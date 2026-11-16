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

#ifndef GITREPOSITORY_H
#define GITREPOSITORY_H

#include <QByteArray>
#include <QDateTime>
#include <QFlags>
#include <QList>
#include <QMap>
#include <QString>

#include <memory>
#include <optional>

struct git_commit;
struct git_oid;
struct git_repository;

/*!
 * \brief Access to a Git repository with libgit2.
 * Does not start a git process. Without libgit2 (OM_OMEDIT_HAVE_LIBGIT2 not set)
 * GitRepository::open never finds a repository.
 */
class GitRepository
{
public:
  enum StatusFlag {
    Current = 0,
    IndexNew = 0x0001,
    IndexModified = 0x0002,
    IndexDeleted = 0x0004,
    IndexRenamed = 0x0008,
    IndexTypeChange = 0x0010,
    WorkTreeNew = 0x0080,
    WorkTreeModified = 0x0100,
    WorkTreeDeleted = 0x0200,
    WorkTreeTypeChange = 0x0400,
    WorkTreeRenamed = 0x0800,
    Ignored = 0x4000,
    Conflicted = 0x8000
  };
  Q_DECLARE_FLAGS(Status, StatusFlag)

  // Line ranges are 1-based. A range with 0 lines is a position: the lines were
  // removed (newLines == 0) or added (oldLines == 0) after that line.
  struct DiffHunk {
    int oldStart;
    int oldLines;
    int newStart;
    int newLines;
  };

  struct Commit {
    QString id;
    QString shortId;
    QString summary;
    QString message;
    QString authorName;
    QString authorEmail;
    QDateTime time;
  };

  static bool isAvailable();
  static GitRepository open(const QString &path);
  static GitRepository init(const QString &directory);
  static std::optional<QString> hashFile(const QString &filePath);

  GitRepository() = default;
  bool isValid() const {return mpRepository != nullptr;}
  QString errorString() const {return mErrorString;}

  QString workingDirectory() const;
  bool contains(const QString &filePath) const;
  QString branchName() const;
  bool isHeadDetached() const;
  QString headCommitId() const;

  std::optional<Status> fileStatus(const QString &filePath) const;
  QMap<QString, Status> changedFiles(bool includeUntracked = true) const;
  std::optional<QByteArray> fileContent(const QString &filePath, const QString &revision = "HEAD") const;
  std::optional<QList<DiffHunk>> diffHunks(const QString &filePath, const QString &revision = "HEAD") const;
  std::optional<QList<DiffHunk>> diffHunksOfContent(const QString &filePath, const QByteArray &content, const QString &revision = "HEAD") const;
  std::optional<QString> diff(const QString &filePath, const QString &revision = "HEAD") const;
  std::optional<QString> diffOfContent(const QString &filePath, const QByteArray &content, const QString &revision = "HEAD") const;
  QString configValue(const QString &name) const;
  std::optional<QString> fileHash(const QString &filePath) const;
  QList<Commit> log(int maxCount = -1, const QString &filePath = QString()) const;
  std::optional<QString> show(const QString &revision) const;

  bool stage(const QString &filePath);
  bool unstage(const QString &filePath);
  std::optional<QString> commit(const QString &message, const QString &authorName = QString(), const QString &authorEmail = QString());
  std::optional<QString> commitFile(const QString &filePath, const QString &message);
  std::optional<QString> revert(const QString &revision);
private:
  std::shared_ptr<git_repository> mpRepository;
  QString mWorkingDirectory;
  mutable QString mErrorString;

  QString relativePath(const QString &filePath) const;
  QString checkedRelativePath(const QString &filePath) const;
  bool fail() const;
  bool isIdle() const;
  int lookupCommit(git_commit **ppCommit, const QString &revision) const;
  std::optional<QString> createCommit(const git_oid *pTreeId, const QString &message, const QString &authorName, const QString &authorEmail);
  std::optional<QByteArray> revisionContent(const QString &relativeFilePath, const QString &revision, bool *pExists) const;
  std::optional<QByteArray> workTreeContent(const QString &filePath) const;
};

Q_DECLARE_OPERATORS_FOR_FLAGS(GitRepository::Status)

#endif // GITREPOSITORY_H
