/*
 * This file is part of OpenModelica.
 *
 * Copyright (c) 1998-2026, Open Source Modelica Consortium (OSMC),
 * c/o Linköpings universitet, Department of Computer and Information Science,
 * SE-58183 Linköping, Sweden.
 *
 * All rights reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF AGPL VERSION 3 LICENSE OR
 * THIS OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.8.
 * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES
 * RECIPIENT'S ACCEPTANCE OF THE OSMC PUBLIC LICENSE OR THE GNU AGPL
 * VERSION 3, ACCORDING TO RECIPIENTS CHOICE.
 *
 * The OpenModelica software and the OSMC (Open Source Modelica Consortium)
 * Public License (OSMC-PL) are obtained from OSMC, either from the above
 * address, from the URLs:
 * http://www.openmodelica.org or
 * https://github.com/OpenModelica/ or
 * http://www.ida.liu.se/projects/OpenModelica,
 * and in the OpenModelica distribution.
 *
 * GNU AGPL version 3 is obtained from:
 * https://www.gnu.org/licenses/licenses.html#GPL
 *
 * This program is distributed WITHOUT ANY WARRANTY; without
 * even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY SET FORTH
 * IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF OSMC-PL.
 *
 * See the full OSMC Public License conditions for more details.
 *
 */

#include "GitRepository.h"

#include <QDir>
#include <QFile>
#include <QFileInfo>
#include <QTimeZone>

#ifdef OM_OMEDIT_HAVE_LIBGIT2
#include <git2.h>

namespace {

constexpr bool sameFlag(GitRepository::StatusFlag flag, git_status_t gitFlag) {return static_cast<int>(flag) == static_cast<int>(gitFlag);}

static_assert(sameFlag(GitRepository::IndexNew, GIT_STATUS_INDEX_NEW)
              && sameFlag(GitRepository::IndexModified, GIT_STATUS_INDEX_MODIFIED)
              && sameFlag(GitRepository::IndexDeleted, GIT_STATUS_INDEX_DELETED)
              && sameFlag(GitRepository::IndexRenamed, GIT_STATUS_INDEX_RENAMED)
              && sameFlag(GitRepository::IndexTypeChange, GIT_STATUS_INDEX_TYPECHANGE)
              && sameFlag(GitRepository::WorkTreeNew, GIT_STATUS_WT_NEW)
              && sameFlag(GitRepository::WorkTreeModified, GIT_STATUS_WT_MODIFIED)
              && sameFlag(GitRepository::WorkTreeDeleted, GIT_STATUS_WT_DELETED)
              && sameFlag(GitRepository::WorkTreeTypeChange, GIT_STATUS_WT_TYPECHANGE)
              && sameFlag(GitRepository::WorkTreeRenamed, GIT_STATUS_WT_RENAMED)
              && sameFlag(GitRepository::Ignored, GIT_STATUS_IGNORED)
              && sameFlag(GitRepository::Conflicted, GIT_STATUS_CONFLICTED),
              "GitRepository::StatusFlag must match git_status_t");

template <typename T, void (*Free)(T*)>
struct GitPtr
{
  T *p = nullptr;
  GitPtr() = default;
  GitPtr(const GitPtr&) = delete;
  GitPtr& operator=(const GitPtr&) = delete;
  ~GitPtr() {if (p) Free(p);}
  T** operator&() {return &p;}
  T* operator->() const {return p;}
  operator T*() const {return p;}
};

struct GitBuf
{
  git_buf buf{};
  ~GitBuf() {git_buf_dispose(&buf);}
};

QString lastGitError()
{
  const git_error *pError = git_error_last();
  return pError && pError->message ? QString::fromUtf8(pError->message) : QStringLiteral("Unknown libgit2 error.");
}

QByteArray toGitPath(const QString &path)
{
  return QDir::fromNativeSeparators(path).toUtf8();
}

int collectHunk(const git_diff_delta*, const git_diff_hunk *pHunk, void *pPayload)
{
  static_cast<QList<GitRepository::DiffHunk>*>(pPayload)->append({pHunk->old_start, pHunk->old_lines, pHunk->new_start, pHunk->new_lines});
  return 0;
}

// git_repository_index caches the index. Read it again in case another program changed it.
int readIndex(git_index **ppIndex, git_repository *pRepository)
{
  int error = git_repository_index(ppIndex, pRepository);
  if (error < 0) {
    return error;
  }
  return git_index_read(*ppIndex, 0);
}

GitRepository::Commit commitInfo(const git_commit *pCommit)
{
  GitRepository::Commit commit;
  commit.id = QString::fromLatin1(git_oid_tostr_s(git_commit_id(pCommit)));
  GitBuf shortId;
  commit.shortId = git_object_short_id(&shortId.buf, reinterpret_cast<const git_object*>(pCommit)) == 0
                   ? QString::fromLatin1(shortId.buf.ptr, static_cast<qsizetype>(shortId.buf.size)) : commit.id.left(7);
  const char *summary = git_commit_summary(const_cast<git_commit*>(pCommit));
  commit.summary = summary ? QString::fromUtf8(summary) : QString();
  commit.message = QString::fromUtf8(git_commit_message(pCommit));
  const git_signature *pAuthor = git_commit_author(pCommit);
  commit.authorName = QString::fromUtf8(pAuthor->name);
  commit.authorEmail = QString::fromUtf8(pAuthor->email);
  commit.time = QDateTime::fromSecsSinceEpoch(pAuthor->when.time, QTimeZone(pAuthor->when.offset * 60));
  return commit;
}

std::optional<git_oid> treeEntryId(const git_commit *pCommit, const char *path)
{
  GitPtr<git_tree, git_tree_free> pTree;
  GitPtr<git_tree_entry, git_tree_entry_free> pEntry;
  if (git_commit_tree(&pTree, pCommit) < 0 || git_tree_entry_bypath(&pEntry, pTree, path) < 0) {
    return std::nullopt;
  }
  return *git_tree_entry_id(pEntry);
}

bool commitChangesPath(const git_commit *pCommit, const char *path)
{
  std::optional<git_oid> id = treeEntryId(pCommit, path);
  if (git_commit_parentcount(pCommit) == 0) {
    return id.has_value();
  }
  GitPtr<git_commit, git_commit_free> pParent;
  if (git_commit_parent(&pParent, pCommit, 0) < 0) {
    return true;
  }
  std::optional<git_oid> parentId = treeEntryId(pParent, path);
  if (id.has_value() != parentId.has_value()) {
    return true;
  }
  return id && !git_oid_equal(&*id, &*parentId);
}

} // namespace
#endif // OM_OMEDIT_HAVE_LIBGIT2

/*!
 * \class GitRepository
 * \brief Access to a Git repository with libgit2.
 */
/*!
 * \brief GitRepository::isAvailable
 * \return true if OMEdit is built with libgit2.
 */
bool GitRepository::isAvailable()
{
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  return true;
#else
  return false;
#endif
}

/*!
 * \brief GitRepository::open
 * Finds the repository that contains a file or directory. Searches the parent directories.
 * \param path - a file or directory in the working tree.
 * \return an invalid GitRepository if path is not in a non-bare repository; errorString() tells why.
 */
GitRepository GitRepository::open(const QString &path)
{
  GitRepository repository;
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  QFileInfo fileInfo(path);
  QString directory = fileInfo.isDir() ? fileInfo.absoluteFilePath() : fileInfo.absolutePath();
  if (git_libgit2_init() < 0) {
    repository.mErrorString = lastGitError();
    return repository;
  }
  git_repository *pRepository = nullptr;
  if (git_repository_open_ext(&pRepository, toGitPath(directory).constData(), 0, nullptr) < 0) {
    repository.mErrorString = lastGitError();
    git_libgit2_shutdown();
    return repository;
  }
  std::shared_ptr<git_repository> pShared(pRepository, [](git_repository *p) {
    git_repository_free(p);
    git_libgit2_shutdown();
  });
  const char *workDirectory = git_repository_workdir(pRepository);
  if (!workDirectory) {
    repository.mErrorString = QStringLiteral("The repository has no working directory.");
    return repository;
  }
  repository.mpRepository = pShared;
  QString workingDirectory = QString::fromUtf8(workDirectory);
  QString canonicalPath = QFileInfo(workingDirectory).canonicalFilePath();
  repository.mWorkingDirectory = canonicalPath.isEmpty() ? QDir::cleanPath(workingDirectory) : canonicalPath;
#else
  Q_UNUSED(path);
  repository.mErrorString = QStringLiteral("OMEdit is built without libgit2.");
#endif
  return repository;
}

/*!
 * \brief GitRepository::init
 * Creates a repository in directory, like git init. An existing repository is kept.
 * \return the repository, or an invalid GitRepository; errorString() tells why.
 */
GitRepository GitRepository::init(const QString &directory)
{
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  if (git_libgit2_init() < 0) {
    GitRepository repository;
    repository.mErrorString = lastGitError();
    return repository;
  }
  git_repository *pRepository = nullptr;
  if (git_repository_init(&pRepository, toGitPath(QFileInfo(directory).absoluteFilePath()).constData(), 0) < 0) {
    GitRepository repository;
    repository.mErrorString = lastGitError();
    git_libgit2_shutdown();
    return repository;
  }
  git_repository_free(pRepository);
  git_libgit2_shutdown();
  return open(directory);
#else
  return open(directory);
#endif
}

/*!
 * \brief GitRepository::hashFile
 * \return the blob hash of the file without filters, like git hash-object --no-filters. Works outside of a repository.
 */
std::optional<QString> GitRepository::hashFile(const QString &filePath)
{
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  if (git_libgit2_init() < 0) {
    return std::nullopt;
  }
  git_oid oid;
  int error = git_odb_hashfile(&oid, toGitPath(QFileInfo(filePath).absoluteFilePath()).constData(), GIT_OBJECT_BLOB);
  std::optional<QString> hash;
  if (error == 0) {
    hash = QString::fromLatin1(git_oid_tostr_s(&oid));
  }
  git_libgit2_shutdown();
  return hash;
#else
  Q_UNUSED(filePath);
  return std::nullopt;
#endif
}

/*!
 * \brief GitRepository::workingDirectory
 * \return the canonical path of the top directory of the working tree, without a trailing slash.
 */
QString GitRepository::workingDirectory() const
{
  return mWorkingDirectory;
}

/*!
 * \brief GitRepository::contains
 * \return true if filePath is in the working tree. Does not check if Git tracks or ignores it.
 */
bool GitRepository::contains(const QString &filePath) const
{
  return isValid() && !relativePath(filePath).isEmpty();
}

/*!
 * \brief GitRepository::branchName
 * \return the short name of the current branch, also of a branch without commits. Empty if HEAD is detached.
 */
QString GitRepository::branchName() const
{
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  if (!isValid()) {
    return QString();
  }
  GitPtr<git_reference, git_reference_free> pHead;
  if (git_reference_lookup(&pHead, mpRepository.get(), "HEAD") < 0) {
    fail();
    return QString();
  }
  if (git_reference_type(pHead) != GIT_REFERENCE_SYMBOLIC) {
    return QString();
  }
  QString target = QString::fromUtf8(git_reference_symbolic_target(pHead));
  const QString prefix = QStringLiteral("refs/heads/");
  return target.startsWith(prefix) ? target.mid(prefix.size()) : target;
#else
  return QString();
#endif
}

/*!
 * \brief GitRepository::isHeadDetached
 */
bool GitRepository::isHeadDetached() const
{
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  return isValid() && git_repository_head_detached(mpRepository.get()) == 1;
#else
  return false;
#endif
}

/*!
 * \brief GitRepository::headCommitId
 * \return the full hash of the HEAD commit. Empty if the branch has no commits.
 */
QString GitRepository::headCommitId() const
{
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  if (!isValid()) {
    return QString();
  }
  git_oid oid;
  if (git_reference_name_to_id(&oid, mpRepository.get(), "HEAD") < 0) {
    fail();
    return QString();
  }
  return QString::fromLatin1(git_oid_tostr_s(&oid));
#else
  return QString();
#endif
}

/*!
 * \brief GitRepository::fileStatus
 * \param filePath - a file in the working tree.
 * \return the status of the file, or nothing if filePath is not in the working tree or is unknown to Git and does not exist.
 */
std::optional<GitRepository::Status> GitRepository::fileStatus(const QString &filePath) const
{
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  QString path = relativePath(filePath);
  if (path.isEmpty()) {
    return std::nullopt;
  }
  unsigned int flags = 0;
  if (git_status_file(&flags, mpRepository.get(), toGitPath(path).constData()) < 0) {
    fail();
    return std::nullopt;
  }
  return Status(static_cast<int>(flags));
#else
  Q_UNUSED(filePath);
  return std::nullopt;
#endif
}

/*!
 * \brief GitRepository::changedFiles
 * \param includeUntracked - also list the untracked files that are not ignored.
 * \return the absolute paths of the files that differ from HEAD, with their status.
 */
QMap<QString, GitRepository::Status> GitRepository::changedFiles(bool includeUntracked) const
{
  QMap<QString, Status> files;
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  if (!isValid()) {
    return files;
  }
  git_status_options options = GIT_STATUS_OPTIONS_INIT;
  options.show = GIT_STATUS_SHOW_INDEX_AND_WORKDIR;
  options.flags = GIT_STATUS_OPT_RENAMES_HEAD_TO_INDEX | GIT_STATUS_OPT_EXCLUDE_SUBMODULES;
  if (includeUntracked) {
    options.flags |= GIT_STATUS_OPT_INCLUDE_UNTRACKED | GIT_STATUS_OPT_RECURSE_UNTRACKED_DIRS;
  }
  GitPtr<git_status_list, git_status_list_free> pList;
  if (git_status_list_new(&pList, mpRepository.get(), &options) < 0) {
    fail();
    return files;
  }
  const QDir workingDirectory(mWorkingDirectory);
  const size_t count = git_status_list_entrycount(pList);
  for (size_t i = 0; i < count; ++i) {
    const git_status_entry *pEntry = git_status_byindex(pList, i);
    const git_diff_delta *pDelta = pEntry->index_to_workdir ? pEntry->index_to_workdir : pEntry->head_to_index;
    if (pDelta && pDelta->new_file.path) {
      files[workingDirectory.absoluteFilePath(QString::fromUtf8(pDelta->new_file.path))] = Status(static_cast<int>(pEntry->status));
    }
  }
#else
  Q_UNUSED(includeUntracked);
#endif
  return files;
}

/*!
 * \brief GitRepository::fileContent
 * \param filePath - a file in the working tree.
 * \param revision - a revision such as "HEAD", "master" or a commit hash.
 * \return the content of the file at revision as Git stores it, or nothing if it does not exist there.
 */
std::optional<QByteArray> GitRepository::fileContent(const QString &filePath, const QString &revision) const
{
  QString path = relativePath(filePath);
  if (path.isEmpty()) {
    return std::nullopt;
  }
  bool exists = false;
  std::optional<QByteArray> content = revisionContent(path, revision, &exists);
  if (!exists) {
    return std::nullopt;
  }
  return content;
}

/*!
 * \brief GitRepository::diffHunks
 * Compares the file on disk with the file at revision. A file that does not exist at revision is all added.
 * \return the changed line ranges, or nothing on an error.
 */
std::optional<QList<GitRepository::DiffHunk>> GitRepository::diffHunks(const QString &filePath, const QString &revision) const
{
  std::optional<QByteArray> content = workTreeContent(filePath);
  if (!content) {
    return std::nullopt;
  }
  return diffHunksOfContent(filePath, *content, revision);
}

/*!
 * \brief GitRepository::diffHunksOfContent
 * Compares content, e.g. the unsaved text of an editor, with the file at revision.
 * \param content - the new content, with the line endings of the working tree.
 * \return the changed line ranges, or nothing on an error.
 */
std::optional<QList<GitRepository::DiffHunk>> GitRepository::diffHunksOfContent(const QString &filePath, const QByteArray &content, const QString &revision) const
{
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  QString path = relativePath(filePath);
  if (path.isEmpty()) {
    return std::nullopt;
  }
  std::optional<QByteArray> oldContent = revisionContent(path, revision, nullptr);
  if (!oldContent) {
    return std::nullopt;
  }
  QByteArray gitPath = toGitPath(path);
  git_diff_options options = GIT_DIFF_OPTIONS_INIT;
  options.context_lines = 0;
  QList<DiffHunk> hunks;
  if (git_diff_buffers(oldContent->constData(), oldContent->size(), gitPath.constData(), content.constData(), content.size(),
                       gitPath.constData(), &options, nullptr, nullptr, collectHunk, nullptr, &hunks) < 0) {
    fail();
    return std::nullopt;
  }
  return hunks;
#else
  Q_UNUSED(filePath);
  Q_UNUSED(content);
  Q_UNUSED(revision);
  return std::nullopt;
#endif
}

/*!
 * \brief GitRepository::diff
 * \return the unified diff of the file at revision and the file on disk, or nothing on an error. Empty if they are equal.
 */
std::optional<QString> GitRepository::diff(const QString &filePath, const QString &revision) const
{
  std::optional<QByteArray> content = workTreeContent(filePath);
  if (!content) {
    return std::nullopt;
  }
  return diffOfContent(filePath, *content, revision);
}

/*!
 * \brief GitRepository::diffOfContent
 * \param content - the new content, with the line endings of the working tree.
 * \return the unified diff of the file at revision and content, or nothing on an error. Empty if they are equal.
 */
std::optional<QString> GitRepository::diffOfContent(const QString &filePath, const QByteArray &content, const QString &revision) const
{
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  QString path = relativePath(filePath);
  if (path.isEmpty()) {
    return std::nullopt;
  }
  std::optional<QByteArray> oldContent = revisionContent(path, revision, nullptr);
  if (!oldContent) {
    return std::nullopt;
  }
  QByteArray gitPath = toGitPath(path);
  git_diff_options options = GIT_DIFF_OPTIONS_INIT;
  GitPtr<git_patch, git_patch_free> pPatch;
  if (git_patch_from_buffers(&pPatch, oldContent->constData(), oldContent->size(), gitPath.constData(),
                             content.constData(), content.size(), gitPath.constData(), &options) < 0) {
    fail();
    return std::nullopt;
  }
  if (git_patch_num_hunks(pPatch) == 0) {
    return QString();
  }
  GitBuf buffer;
  if (git_patch_to_buf(&buffer.buf, pPatch) < 0) {
    fail();
    return std::nullopt;
  }
  return QString::fromUtf8(buffer.buf.ptr, static_cast<qsizetype>(buffer.buf.size));
#else
  Q_UNUSED(filePath);
  Q_UNUSED(content);
  Q_UNUSED(revision);
  return std::nullopt;
#endif
}

/*!
 * \brief GitRepository::configValue
 * \return the value of a configuration variable, e.g. user.name, from the repository, global and system configuration.
 */
QString GitRepository::configValue(const QString &name) const
{
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  if (!isValid()) {
    return QString();
  }
  GitPtr<git_config, git_config_free> pConfig;
  if (git_repository_config_snapshot(&pConfig, mpRepository.get()) < 0) {
    fail();
    return QString();
  }
  const char *value = nullptr;
  int error = git_config_get_string(&value, pConfig, name.toUtf8().constData());
  if (error < 0) {
    if (error != GIT_ENOTFOUND) {
      fail();
    }
    return QString();
  }
  return QString::fromUtf8(value);
#else
  Q_UNUSED(name);
  return QString();
#endif
}

/*!
 * \brief GitRepository::fileHash
 * \return the blob hash of the file on disk with the filters of the repository applied, like git hash-object.
 */
std::optional<QString> GitRepository::fileHash(const QString &filePath) const
{
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  QString path = checkedRelativePath(filePath);
  if (path.isEmpty()) {
    return std::nullopt;
  }
  git_oid oid;
  if (git_repository_hashfile(&oid, mpRepository.get(), toGitPath(QFileInfo(filePath).absoluteFilePath()).constData(), GIT_OBJECT_BLOB,
                              toGitPath(path).constData()) < 0) {
    fail();
    return std::nullopt;
  }
  return QString::fromLatin1(git_oid_tostr_s(&oid));
#else
  Q_UNUSED(filePath);
  return std::nullopt;
#endif
}

/*!
 * \brief GitRepository::log
 * \param maxCount - the maximum number of commits, or -1 for all.
 * \param filePath - if not empty, only the commits that change this file compared to their first parent.
 * \return the commits reachable from HEAD, newest first.
 */
QList<GitRepository::Commit> GitRepository::log(int maxCount, const QString &filePath) const
{
  QList<Commit> commits;
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  if (!isValid() || git_repository_head_unborn(mpRepository.get()) == 1) {
    return commits;
  }
  QByteArray gitPath;
  if (!filePath.isEmpty()) {
    QString path = checkedRelativePath(filePath);
    if (path.isEmpty()) {
      return commits;
    }
    gitPath = toGitPath(path);
  }
  GitPtr<git_revwalk, git_revwalk_free> pWalk;
  if (git_revwalk_new(&pWalk, mpRepository.get()) < 0 || git_revwalk_sorting(pWalk, GIT_SORT_TOPOLOGICAL | GIT_SORT_TIME) < 0 || git_revwalk_push_head(pWalk) < 0) {
    fail();
    return commits;
  }
  git_oid oid;
  int error = 0;
  while ((maxCount < 0 || commits.size() < maxCount) && (error = git_revwalk_next(&oid, pWalk)) == 0) {
    GitPtr<git_commit, git_commit_free> pCommit;
    if (git_commit_lookup(&pCommit, mpRepository.get(), &oid) < 0) {
      fail();
      return commits;
    }
    if (gitPath.isEmpty() || commitChangesPath(pCommit, gitPath.constData())) {
      commits.append(commitInfo(pCommit));
    }
  }
  if (error < 0 && error != GIT_ITEROVER) {
    fail();
  }
#else
  Q_UNUSED(maxCount);
  Q_UNUSED(filePath);
#endif
  return commits;
}

/*!
 * \brief GitRepository::show
 * \return the commit with its message and the changed files, like git show --stat=80.
 */
std::optional<QString> GitRepository::show(const QString &revision) const
{
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  if (!isValid()) {
    return std::nullopt;
  }
  GitPtr<git_commit, git_commit_free> pCommit;
  if (lookupCommit(&pCommit, revision) < 0) {
    fail();
    return std::nullopt;
  }
  GitPtr<git_commit, git_commit_free> pParent;
  GitPtr<git_tree, git_tree_free> pParentTree;
  GitPtr<git_tree, git_tree_free> pTree;
  if ((git_commit_parentcount(pCommit) > 0 && (git_commit_parent(&pParent, pCommit, 0) < 0 || git_commit_tree(&pParentTree, pParent) < 0))
      || git_commit_tree(&pTree, pCommit) < 0) {
    fail();
    return std::nullopt;
  }
  git_diff_options options = GIT_DIFF_OPTIONS_INIT;
  GitPtr<git_diff, git_diff_free> pDiff;
  GitPtr<git_diff_stats, git_diff_stats_free> pStats;
  GitBuf stats;
  if (git_diff_tree_to_tree(&pDiff, mpRepository.get(), pParentTree, pTree, &options) < 0 || git_diff_get_stats(&pStats, pDiff) < 0
      || git_diff_stats_to_buf(&stats.buf, pStats, GIT_DIFF_STATS_FULL, 80) < 0) {
    fail();
    return std::nullopt;
  }
  Commit commit = commitInfo(pCommit);
  QString message;
  for (const QString &line : commit.message.trimmed().split('\n')) {
    message += (line.isEmpty() ? QString() : QStringLiteral("    ") + line) + '\n';
  }
  return QStringLiteral("commit %1\nAuthor: %2 <%3>\nDate:   %4\n\n%5\n%6")
      .arg(commit.id, commit.authorName, commit.authorEmail, commit.time.toString(Qt::RFC2822Date), message,
           QString::fromUtf8(stats.buf.ptr, static_cast<qsizetype>(stats.buf.size)));
#else
  Q_UNUSED(revision);
  return std::nullopt;
#endif
}

/*!
 * \brief GitRepository::stage
 * Adds the file to the index, or removes it from the index if it does not exist, like git add.
 */
bool GitRepository::stage(const QString &filePath)
{
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  QString path = checkedRelativePath(filePath);
  if (path.isEmpty()) {
    return false;
  }
  QByteArray gitPath = toGitPath(path);
  GitPtr<git_index, git_index_free> pIndex;
  if (readIndex(&pIndex, mpRepository.get()) < 0) {
    return fail();
  }
  QFileInfo fileInfo(filePath);
  int error = (fileInfo.exists() || fileInfo.isSymLink()) ? git_index_add_bypath(pIndex, gitPath.constData())
                                                         : git_index_remove_bypath(pIndex, gitPath.constData());
  if (error < 0 || git_index_write(pIndex) < 0) {
    return fail();
  }
  return true;
#else
  Q_UNUSED(filePath);
  return false;
#endif
}

/*!
 * \brief GitRepository::unstage
 * Sets the index entry of the file back to HEAD, like git reset HEAD -- file. Does not change the file on disk.
 */
bool GitRepository::unstage(const QString &filePath)
{
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  QString path = checkedRelativePath(filePath);
  if (path.isEmpty()) {
    return false;
  }
  QByteArray gitPath = toGitPath(path);
  if (git_repository_head_unborn(mpRepository.get()) == 1) {
    GitPtr<git_index, git_index_free> pIndex;
    if (readIndex(&pIndex, mpRepository.get()) < 0 || git_index_remove_bypath(pIndex, gitPath.constData()) < 0
        || git_index_write(pIndex) < 0) {
      return fail();
    }
    return true;
  }
  GitPtr<git_index, git_index_free> pIndex;
  GitPtr<git_object, git_object_free> pHead;
  if (readIndex(&pIndex, mpRepository.get()) < 0 || git_revparse_single(&pHead, mpRepository.get(), "HEAD") < 0) {
    return fail();
  }
  char *paths[] = {gitPath.data()};
  git_strarray pathSpec = {paths, 1};
  if (git_reset_default(mpRepository.get(), pHead, &pathSpec) < 0) {
    return fail();
  }
  return true;
#else
  Q_UNUSED(filePath);
  return false;
#endif
}

/*!
 * \brief GitRepository::commit
 * Commits the index, like git commit. Without authorName and authorEmail the author is user.name and user.email.
 * \return the hash of the new commit, or nothing on an error, e.g. if there is nothing to commit.
 */
std::optional<QString> GitRepository::commit(const QString &message, const QString &authorName, const QString &authorEmail)
{
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  if (!isIdle()) {
    return std::nullopt;
  }
  GitPtr<git_index, git_index_free> pIndex;
  if (readIndex(&pIndex, mpRepository.get()) < 0) {
    fail();
    return std::nullopt;
  }
  if (git_index_has_conflicts(pIndex)) {
    mErrorString = QStringLiteral("The index has conflicts.");
    return std::nullopt;
  }
  git_oid treeId;
  if (git_index_write_tree(&treeId, pIndex) < 0) {
    fail();
    return std::nullopt;
  }
  return createCommit(&treeId, message, authorName, authorEmail);
#else
  Q_UNUSED(message);
  Q_UNUSED(authorName);
  Q_UNUSED(authorEmail);
  return std::nullopt;
#endif
}

/*!
 * \brief GitRepository::commitFile
 * Stages the file and commits only this file, like git add file followed by git commit file.
 * Other staged changes stay staged.
 * \return the hash of the new commit, or nothing on an error, e.g. if the file is not changed.
 */
std::optional<QString> GitRepository::commitFile(const QString &filePath, const QString &message)
{
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  if (!isIdle() || !stage(filePath)) {
    return std::nullopt;
  }
  QByteArray gitPath = toGitPath(relativePath(filePath));
  GitPtr<git_index, git_index_free> pIndex;
  if (readIndex(&pIndex, mpRepository.get()) < 0) {
    fail();
    return std::nullopt;
  }
  GitPtr<git_tree, git_tree_free> pBaseline;
  if (git_repository_head_unborn(mpRepository.get()) == 1) {
    GitPtr<git_treebuilder, git_treebuilder_free> pBuilder;
    git_oid emptyTreeId;
    if (git_treebuilder_new(&pBuilder, mpRepository.get(), nullptr) < 0 || git_treebuilder_write(&emptyTreeId, pBuilder) < 0
        || git_tree_lookup(&pBaseline, mpRepository.get(), &emptyTreeId) < 0) {
      fail();
      return std::nullopt;
    }
  } else {
    GitPtr<git_commit, git_commit_free> pHead;
    if (lookupCommit(&pHead, QStringLiteral("HEAD")) < 0 || git_commit_tree(&pBaseline, pHead) < 0) {
      fail();
      return std::nullopt;
    }
  }
  git_tree_update update{};
  update.path = gitPath.constData();
  if (const git_index_entry *pEntry = git_index_get_bypath(pIndex, gitPath.constData(), 0)) {
    update.action = GIT_TREE_UPDATE_UPSERT;
    update.id = pEntry->id;
    update.filemode = static_cast<git_filemode_t>(pEntry->mode);
  } else {
    update.action = GIT_TREE_UPDATE_REMOVE;
  }
  git_oid treeId;
  if (git_tree_create_updated(&treeId, mpRepository.get(), pBaseline, 1, &update) < 0) {
    fail();
    return std::nullopt;
  }
  return createCommit(&treeId, message, QString(), QString());
#else
  Q_UNUSED(filePath);
  Q_UNUSED(message);
  return std::nullopt;
#endif
}

/*!
 * \brief GitRepository::revert
 * Makes a new commit that reverts the changes of a commit, like git revert --no-edit.
 * Changes nothing if the revert conflicts with later commits or with local changes of the reverted files.
 * \return the hash of the new commit, or nothing on an error.
 */
std::optional<QString> GitRepository::revert(const QString &revision)
{
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  if (!isIdle()) {
    return std::nullopt;
  }
  GitPtr<git_commit, git_commit_free> pCommit;
  GitPtr<git_commit, git_commit_free> pHead;
  if (lookupCommit(&pCommit, revision) < 0 || lookupCommit(&pHead, QStringLiteral("HEAD")) < 0) {
    fail();
    return std::nullopt;
  }
  GitPtr<git_index, git_index_free> pIndex;
  if (git_revert_commit(&pIndex, mpRepository.get(), pCommit, pHead, 0, nullptr) < 0) {
    fail();
    return std::nullopt;
  }
  if (git_index_has_conflicts(pIndex)) {
    mErrorString = QStringLiteral("Reverting %1 conflicts with later changes.").arg(revision);
    return std::nullopt;
  }
  git_oid treeId;
  GitPtr<git_tree, git_tree_free> pTree;
  if (git_index_write_tree_to(&treeId, pIndex, mpRepository.get()) < 0 || git_tree_lookup(&pTree, mpRepository.get(), &treeId) < 0) {
    fail();
    return std::nullopt;
  }
  if (git_oid_equal(&treeId, git_commit_tree_id(pHead))) {
    mErrorString = QStringLiteral("Nothing to revert.");
    return std::nullopt;
  }
  // Check the signature before the working tree is changed.
  GitPtr<git_signature, git_signature_free> pSignature;
  if (git_signature_default(&pSignature, mpRepository.get()) < 0) {
    fail();
    return std::nullopt;
  }
  git_checkout_options options = GIT_CHECKOUT_OPTIONS_INIT;
  options.checkout_strategy = GIT_CHECKOUT_SAFE;
  if (git_checkout_tree(mpRepository.get(), reinterpret_cast<git_object*>(pTree.p), &options) < 0) {
    fail();
    return std::nullopt;
  }
  Commit commit = commitInfo(pCommit);
  return createCommit(&treeId, QStringLiteral("Revert \"%1\"\n\nThis reverts commit %2.\n").arg(commit.summary, commit.id), QString(), QString());
#else
  Q_UNUSED(revision);
  return std::nullopt;
#endif
}

/*!
 * \brief GitRepository::relativePath
 * \return filePath relative to the working directory, with '/' separators. Empty if it is not in the working tree.
 */
QString GitRepository::relativePath(const QString &filePath) const
{
  if (!isValid() || filePath.isEmpty()) {
    return QString();
  }
  QFileInfo fileInfo(filePath);
  QString canonicalDirectory = QFileInfo(fileInfo.absolutePath()).canonicalFilePath();
  QString absolutePath = canonicalDirectory.isEmpty() ? fileInfo.absoluteFilePath() : QDir(canonicalDirectory).filePath(fileInfo.fileName());
  QString path = QDir(mWorkingDirectory).relativeFilePath(absolutePath);
  if (path.isEmpty() || path == QLatin1String(".") || path.startsWith(QLatin1String("../")) || path == QLatin1String("..")
      || QDir::isAbsolutePath(path) || path == QLatin1String(".git") || path.startsWith(QLatin1String(".git/"))) {
    return QString();
  }
  return path;
}

/*!
 * \brief GitRepository::checkedRelativePath
 * Like relativePath, but sets errorString() if filePath is not in the working tree.
 */
QString GitRepository::checkedRelativePath(const QString &filePath) const
{
  if (!isValid()) {
    return QString();
  }
  QString path = relativePath(filePath);
  if (path.isEmpty()) {
    mErrorString = QStringLiteral("%1 is not in the working tree of %2.").arg(filePath, mWorkingDirectory);
  }
  return path;
}

/*!
 * \brief GitRepository::isIdle
 * \return true if no merge, revert, rebase or similar is in progress. Sets errorString() otherwise.
 */
bool GitRepository::isIdle() const
{
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  if (!isValid()) {
    return false;
  }
  if (git_repository_state(mpRepository.get()) != GIT_REPOSITORY_STATE_NONE) {
    mErrorString = QStringLiteral("A merge, revert, rebase or similar operation is in progress in %1.").arg(mWorkingDirectory);
    return false;
  }
  return true;
#else
  return false;
#endif
}

/*!
 * \brief GitRepository::lookupCommit
 * \return 0 and the commit of revision in ppCommit, or a libgit2 error code.
 */
int GitRepository::lookupCommit(git_commit **ppCommit, const QString &revision) const
{
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  GitPtr<git_object, git_object_free> pObject;
  int error = git_revparse_single(&pObject, mpRepository.get(), revision.toUtf8().constData());
  if (error < 0) {
    return error;
  }
  return git_object_peel(reinterpret_cast<git_object**>(ppCommit), pObject, GIT_OBJECT_COMMIT);
#else
  Q_UNUSED(ppCommit);
  Q_UNUSED(revision);
  return -1;
#endif
}

/*!
 * \brief GitRepository::createCommit
 * Commits the tree on top of HEAD and moves the current branch to the new commit.
 * \return the hash of the new commit, or nothing on an error, e.g. if the tree is the same as in HEAD.
 */
std::optional<QString> GitRepository::createCommit(const git_oid *pTreeId, const QString &message, const QString &authorName, const QString &authorEmail)
{
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  GitBuf cleanMessage;
  if (git_message_prettify(&cleanMessage.buf, message.toUtf8().constData(), 0, '#') < 0) {
    fail();
    return std::nullopt;
  }
  if (cleanMessage.buf.size == 0) {
    mErrorString = QStringLiteral("The commit message is empty.");
    return std::nullopt;
  }
  GitPtr<git_tree, git_tree_free> pTree;
  if (git_tree_lookup(&pTree, mpRepository.get(), pTreeId) < 0) {
    fail();
    return std::nullopt;
  }
  GitPtr<git_commit, git_commit_free> pParent;
  if (git_repository_head_unborn(mpRepository.get()) == 1) {
    if (git_tree_entrycount(pTree) == 0) {
      mErrorString = QStringLiteral("Nothing to commit.");
      return std::nullopt;
    }
  } else if (lookupCommit(&pParent, QStringLiteral("HEAD")) < 0) {
    fail();
    return std::nullopt;
  } else if (git_oid_equal(git_commit_tree_id(pParent), pTreeId)) {
    mErrorString = QStringLiteral("Nothing to commit.");
    return std::nullopt;
  }
  GitPtr<git_signature, git_signature_free> pAuthor;
  if ((!authorName.isEmpty() || !authorEmail.isEmpty())
      && git_signature_now(&pAuthor, authorName.toUtf8().constData(), authorEmail.toUtf8().constData()) < 0) {
    fail();
    return std::nullopt;
  }
  GitPtr<git_signature, git_signature_free> pCommitter;
  if (git_signature_default(&pCommitter, mpRepository.get()) < 0 && !pAuthor) {
    fail();
    return std::nullopt;
  }
  const git_signature *pAuthorSignature = pAuthor ? pAuthor.p : pCommitter.p;
  const git_signature *pCommitterSignature = pCommitter ? pCommitter.p : pAuthor.p;
  git_oid commitId;
  int error = pParent ? git_commit_create_v(&commitId, mpRepository.get(), "HEAD", pAuthorSignature, pCommitterSignature, nullptr,
                                            cleanMessage.buf.ptr, pTree, 1, pParent.p)
                      : git_commit_create_v(&commitId, mpRepository.get(), "HEAD", pAuthorSignature, pCommitterSignature, nullptr,
                                            cleanMessage.buf.ptr, pTree, 0);
  if (error < 0) {
    fail();
    return std::nullopt;
  }
  return QString::fromLatin1(git_oid_tostr_s(&commitId));
#else
  Q_UNUSED(pTreeId);
  Q_UNUSED(message);
  Q_UNUSED(authorName);
  Q_UNUSED(authorEmail);
  return std::nullopt;
#endif
}

/*!
 * \brief GitRepository::fail
 * Stores the last libgit2 error in errorString().
 * \return false
 */
bool GitRepository::fail() const
{
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  mErrorString = lastGitError();
#endif
  return false;
}

/*!
 * \brief GitRepository::revisionContent
 * \return the file at revision with the line endings of the working tree. Empty if the file does not exist at revision.
 * Nothing on an error, e.g. an unknown revision.
 */
std::optional<QByteArray> GitRepository::revisionContent(const QString &relativeFilePath, const QString &revision, bool *pExists) const
{
  if (pExists) {
    *pExists = false;
  }
#ifdef OM_OMEDIT_HAVE_LIBGIT2
  if (revision == QLatin1String("HEAD") && git_repository_head_unborn(mpRepository.get()) == 1) {
    return QByteArray();
  }
  GitPtr<git_object, git_object_free> pObject;
  if (git_revparse_single(&pObject, mpRepository.get(), revision.toUtf8().constData()) < 0) {
    fail();
    return std::nullopt;
  }
  GitPtr<git_object, git_object_free> pTree;
  if (git_object_peel(&pTree, pObject, GIT_OBJECT_TREE) < 0) {
    fail();
    return std::nullopt;
  }
  QByteArray gitPath = toGitPath(relativeFilePath);
  GitPtr<git_tree_entry, git_tree_entry_free> pEntry;
  int error = git_tree_entry_bypath(&pEntry, reinterpret_cast<git_tree*>(pTree.p), gitPath.constData());
  if (error == GIT_ENOTFOUND) {
    return QByteArray();
  } else if (error < 0) {
    fail();
    return std::nullopt;
  }
  if (git_tree_entry_type(pEntry) != GIT_OBJECT_BLOB) {
    mErrorString = QStringLiteral("%1 is not a file at %2.").arg(relativeFilePath, revision);
    return std::nullopt;
  }
  GitPtr<git_blob, git_blob_free> pBlob;
  if (git_blob_lookup(&pBlob, mpRepository.get(), git_tree_entry_id(pEntry)) < 0) {
    fail();
    return std::nullopt;
  }
  // Apply the checkout filters, e.g. core.autocrlf, so that the content compares with the file on disk.
  git_blob_filter_options options = GIT_BLOB_FILTER_OPTIONS_INIT;
  GitBuf buffer;
  if (git_blob_filter(&buffer.buf, pBlob, gitPath.constData(), &options) < 0) {
    fail();
    return std::nullopt;
  }
  if (pExists) {
    *pExists = true;
  }
  return QByteArray(buffer.buf.ptr, static_cast<qsizetype>(buffer.buf.size));
#else
  Q_UNUSED(relativeFilePath);
  Q_UNUSED(revision);
  return std::nullopt;
#endif
}

/*!
 * \brief GitRepository::workTreeContent
 * \return the content of the file on disk. Empty if the file does not exist. Nothing if it cannot be read.
 */
std::optional<QByteArray> GitRepository::workTreeContent(const QString &filePath) const
{
  QFile file(filePath);
  if (!file.exists()) {
    return QByteArray();
  }
  if (!file.open(QIODevice::ReadOnly)) {
    mErrorString = file.errorString();
    return std::nullopt;
  }
  return file.readAll();
}
