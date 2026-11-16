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

// The tests make and check the repositories with the git program.

#include "Git/GitRepository.h"

#include <QDir>
#include <QFile>
#include <QProcess>
#include <QStandardPaths>
#include <QTemporaryDir>
#include <QtTest>

class GitRepositoryTest : public QObject
{
  Q_OBJECT
private:
  QTemporaryDir mTemporaryDir;
  QString mGit;
  QString mRepositoryPath;

  QString git(const QStringList &arguments, const QString &directory = QString())
  {
    QProcess process;
    process.setWorkingDirectory(directory.isEmpty() ? mRepositoryPath : directory);
    process.start(mGit, QStringList() << "-c" << "user.name=OMEdit Test" << "-c" << "user.email=test@example.com"
                                      << "-c" << "commit.gpgsign=false" << arguments);
    if (!process.waitForFinished(30000) || process.exitStatus() != QProcess::NormalExit || process.exitCode() != 0) {
      qWarning() << "git" << arguments << "failed:" << process.readAllStandardError();
      return QString();
    }
    return QString::fromUtf8(process.readAllStandardOutput()).trimmed();
  }
  QString filePath(const QString &name) const {return QDir(mRepositoryPath).filePath(name);}
  void writeFile(const QString &name, const QByteArray &content)
  {
    QFile file(filePath(name));
    QDir().mkpath(QFileInfo(file).absolutePath());
    QVERIFY(file.open(QIODevice::WriteOnly | QIODevice::Truncate));
    file.write(content);
  }
  void initRepository(const QString &name)
  {
    mRepositoryPath = QDir(mTemporaryDir.path()).filePath(name);
    QVERIFY(QDir().mkpath(mRepositoryPath));
    git({"init", "-q"});
    git({"symbolic-ref", "HEAD", "refs/heads/main"});
    git({"config", "core.autocrlf", "false"});
    git({"config", "user.name", "OMEdit Test"});
    git({"config", "user.email", "test@example.com"});
  }
  void commitFiles(const QStringList &names, const QString &message)
  {
    git(QStringList() << "add" << names);
    git({"commit", "-q", "-m", message});
  }
private slots:
  void initTestCase();
  void emptyRepository();
  void branchAndHead();
  void fileStatus();
  void changedFiles();
  void diff();
  void diffAgainstRevision();
  void lineEndings();
  void outsideOfRepository();
  void init();
  void configAndHash();
  void stageAndUnstage();
  void commit();
  void commitFile();
  void logAndShow();
  void revert();
  void revertConflict();
};

void GitRepositoryTest::initTestCase()
{
  if (!GitRepository::isAvailable()) {
    QSKIP("OMEdit is built without libgit2.");
  }
  mGit = QStandardPaths::findExecutable("git");
  if (mGit.isEmpty()) {
    QSKIP("The git program is needed to make the test repositories.");
  }
  QVERIFY(mTemporaryDir.isValid());
}

void GitRepositoryTest::emptyRepository()
{
  initRepository("empty");
  writeFile("M.mo", "model M\nend M;\n");
  GitRepository repository = GitRepository::open(filePath("M.mo"));
  QVERIFY2(repository.isValid(), qPrintable(repository.errorString()));
  QCOMPARE(repository.workingDirectory(), QFileInfo(mRepositoryPath).canonicalFilePath());
  QCOMPARE(repository.branchName(), "main");
  QVERIFY(!repository.isHeadDetached());
  QVERIFY(repository.headCommitId().isEmpty());
  QCOMPARE(repository.fileStatus(filePath("M.mo")), std::optional<GitRepository::Status>(GitRepository::WorkTreeNew));
  QVERIFY(!repository.fileContent(filePath("M.mo")));
  std::optional<QList<GitRepository::DiffHunk>> hunks = repository.diffHunks(filePath("M.mo"));
  QVERIFY2(hunks, qPrintable(repository.errorString()));
  QCOMPARE(hunks->size(), 1);
  QCOMPARE(hunks->at(0).oldLines, 0);
  QCOMPARE(hunks->at(0).newStart, 1);
  QCOMPARE(hunks->at(0).newLines, 2);
}

void GitRepositoryTest::branchAndHead()
{
  initRepository("head");
  writeFile("P/package.mo", "package P\nend P;\n");
  git({"add", "P/package.mo"});
  git({"commit", "-q", "-m", "First"});
  git({"switch", "-q", "-c", "feature/x"});
  // Open from a directory below the top of the working tree.
  GitRepository repository = GitRepository::open(filePath("P"));
  QVERIFY2(repository.isValid(), qPrintable(repository.errorString()));
  QCOMPARE(repository.branchName(), QString("feature/x"));
  QCOMPARE(repository.headCommitId(), git({"rev-parse", "HEAD"}));
  git({"switch", "-q", "--detach", "HEAD"});
  QVERIFY(repository.isHeadDetached());
  QVERIFY(repository.branchName().isEmpty());
  QCOMPARE(repository.headCommitId(), git({"rev-parse", "HEAD"}));
}

void GitRepositoryTest::fileStatus()
{
  initRepository("status");
  writeFile(".gitignore", "*.log\n");
  writeFile("A.mo", "model A\nend A;\n");
  writeFile("B.mo", "model B\nend B;\n");
  writeFile("C.mo", "model C\nend C;\n");
  git({"add", ".gitignore", "A.mo", "B.mo", "C.mo"});
  git({"commit", "-q", "-m", "First"});
  writeFile("B.mo", "model B\n  Real x;\nend B;\n");
  QVERIFY(QFile::remove(filePath("C.mo")));
  writeFile("D.mo", "model D\nend D;\n");
  git({"add", "D.mo"});
  writeFile("E.mo", "model E\nend E;\n");
  writeFile("out.log", "log\n");

  GitRepository repository = GitRepository::open(mRepositoryPath);
  QVERIFY2(repository.isValid(), qPrintable(repository.errorString()));
  QCOMPARE(repository.fileStatus(filePath("A.mo")), std::optional<GitRepository::Status>(GitRepository::Current));
  QCOMPARE(repository.fileStatus(filePath("B.mo")), std::optional<GitRepository::Status>(GitRepository::WorkTreeModified));
  QCOMPARE(repository.fileStatus(filePath("C.mo")), std::optional<GitRepository::Status>(GitRepository::WorkTreeDeleted));
  QCOMPARE(repository.fileStatus(filePath("D.mo")), std::optional<GitRepository::Status>(GitRepository::IndexNew));
  QCOMPARE(repository.fileStatus(filePath("E.mo")), std::optional<GitRepository::Status>(GitRepository::WorkTreeNew));
  QCOMPARE(repository.fileStatus(filePath("out.log")), std::optional<GitRepository::Status>(GitRepository::Ignored));
  QVERIFY(!repository.fileStatus(filePath("Missing.mo")));
  QCOMPARE(repository.fileContent(filePath("B.mo")), std::optional<QByteArray>("model B\nend B;\n"));
}

void GitRepositoryTest::changedFiles()
{
  initRepository("changed");
  writeFile("A.mo", "model A\nend A;\n");
  writeFile("B.mo", "model B\nend B;\n");
  git({"add", "A.mo", "B.mo"});
  git({"commit", "-q", "-m", "First"});
  writeFile("B.mo", "model B\n  Real x;\nend B;\n");
  writeFile("Sub/E.mo", "model E\nend E;\n");

  GitRepository repository = GitRepository::open(mRepositoryPath);
  QVERIFY2(repository.isValid(), qPrintable(repository.errorString()));
  const QString directory = repository.workingDirectory();
  QMap<QString, GitRepository::Status> files = repository.changedFiles();
  QCOMPARE(files.size(), 2);
  QCOMPARE(files.value(directory + "/B.mo"), GitRepository::Status(GitRepository::WorkTreeModified));
  QCOMPARE(files.value(directory + "/Sub/E.mo"), GitRepository::Status(GitRepository::WorkTreeNew));
  files = repository.changedFiles(false);
  QCOMPARE(files.keys(), QStringList() << directory + "/B.mo");
}

void GitRepositoryTest::diff()
{
  initRepository("diff");
  writeFile("M.mo", "model M\n  Real a;\n  Real b;\n  Real c;\nend M;\n");
  git({"add", "M.mo"});
  git({"commit", "-q", "-m", "First"});
  writeFile("M.mo", "model M\n  Real a;\n  Real B;\n  Real c;\n  Real d;\nend M;\n");

  GitRepository repository = GitRepository::open(mRepositoryPath);
  QVERIFY2(repository.isValid(), qPrintable(repository.errorString()));
  std::optional<QList<GitRepository::DiffHunk>> hunks = repository.diffHunks(filePath("M.mo"));
  QVERIFY2(hunks, qPrintable(repository.errorString()));
  QCOMPARE(hunks->size(), 2);
  QCOMPARE(hunks->at(0).oldStart, 3);
  QCOMPARE(hunks->at(0).oldLines, 1);
  QCOMPARE(hunks->at(0).newStart, 3);
  QCOMPARE(hunks->at(0).newLines, 1);
  QCOMPARE(hunks->at(1).oldStart, 4);
  QCOMPARE(hunks->at(1).oldLines, 0);
  QCOMPARE(hunks->at(1).newStart, 5);
  QCOMPARE(hunks->at(1).newLines, 1);

  std::optional<QString> patch = repository.diff(filePath("M.mo"));
  QVERIFY2(patch, qPrintable(repository.errorString()));
  QVERIFY(patch->contains("--- a/M.mo\n+++ b/M.mo\n"));
  QVERIFY(patch->contains("\n-  Real b;\n+  Real B;\n"));
  QVERIFY(patch->contains("\n+  Real d;\n"));

  // Unsaved editor text: line 2 removed.
  hunks = repository.diffHunksOfContent(filePath("M.mo"), "model M\n  Real b;\n  Real c;\nend M;\n");
  QVERIFY2(hunks, qPrintable(repository.errorString()));
  QCOMPARE(hunks->size(), 1);
  QCOMPARE(hunks->at(0).oldStart, 2);
  QCOMPARE(hunks->at(0).oldLines, 1);
  QCOMPARE(hunks->at(0).newStart, 1);
  QCOMPARE(hunks->at(0).newLines, 0);

  patch = repository.diffOfContent(filePath("M.mo"), "model M\n  Real a;\n  Real b;\n  Real c;\nend M;\n");
  QCOMPARE(patch, std::optional<QString>(QString()));

  QVERIFY(QFile::remove(filePath("M.mo")));
  hunks = repository.diffHunks(filePath("M.mo"));
  QVERIFY2(hunks, qPrintable(repository.errorString()));
  QCOMPARE(hunks->size(), 1);
  QCOMPARE(hunks->at(0).oldLines, 5);
  QCOMPARE(hunks->at(0).newLines, 0);
}

void GitRepositoryTest::diffAgainstRevision()
{
  initRepository("revision");
  writeFile("M.mo", "model M\nend M;\n");
  git({"add", "M.mo"});
  git({"commit", "-q", "-m", "First"});
  git({"switch", "-q", "-c", "feature"});
  writeFile("M.mo", "model M\n  Real x;\nend M;\n");
  git({"commit", "-q", "-a", "-m", "Second"});

  GitRepository repository = GitRepository::open(mRepositoryPath);
  QVERIFY2(repository.isValid(), qPrintable(repository.errorString()));
  std::optional<QList<GitRepository::DiffHunk>> hunks = repository.diffHunks(filePath("M.mo"));
  QVERIFY2(hunks, qPrintable(repository.errorString()));
  QVERIFY(hunks->isEmpty());
  hunks = repository.diffHunks(filePath("M.mo"), "main");
  QVERIFY2(hunks, qPrintable(repository.errorString()));
  QCOMPARE(hunks->size(), 1);
  QCOMPARE(hunks->at(0).newStart, 2);
  QCOMPARE(hunks->at(0).newLines, 1);
  QCOMPARE(repository.fileContent(filePath("M.mo"), "main"), std::optional<QByteArray>("model M\nend M;\n"));
  QVERIFY(!repository.diffHunks(filePath("M.mo"), "no-such-branch"));
  QVERIFY(!repository.errorString().isEmpty());
}

void GitRepositoryTest::lineEndings()
{
  initRepository("crlf");
  git({"config", "core.autocrlf", "true"});
  writeFile("M.mo", "model M\r\nend M;\r\n");
  git({"add", "M.mo"});
  git({"commit", "-q", "-m", "First"});

  GitRepository repository = GitRepository::open(mRepositoryPath);
  QVERIFY2(repository.isValid(), qPrintable(repository.errorString()));
  QCOMPARE(repository.fileStatus(filePath("M.mo")), std::optional<GitRepository::Status>(GitRepository::Current));
  QCOMPARE(repository.fileContent(filePath("M.mo")), std::optional<QByteArray>("model M\r\nend M;\r\n"));
  std::optional<QList<GitRepository::DiffHunk>> hunks = repository.diffHunks(filePath("M.mo"));
  QVERIFY2(hunks, qPrintable(repository.errorString()));
  QVERIFY(hunks->isEmpty());
}

void GitRepositoryTest::outsideOfRepository()
{
  initRepository("inside");
  const QString outside = QDir(mTemporaryDir.path()).filePath("outside.mo");
  QFile file(outside);
  QVERIFY(file.open(QIODevice::WriteOnly));
  file.close();
  GitRepository repository = GitRepository::open(mRepositoryPath);
  QVERIFY2(repository.isValid(), qPrintable(repository.errorString()));
  QVERIFY(repository.contains(filePath("M.mo")));
  QVERIFY(!repository.contains(outside));
  QVERIFY(!repository.contains(filePath(".git/config")));
  QVERIFY(!repository.fileStatus(outside));
  QVERIFY(!repository.diffHunks(outside));

  QVERIFY(!GitRepository().isValid());
  QVERIFY(!GitRepository().fileStatus(outside));
}

void GitRepositoryTest::init()
{
  const QString directory = QDir(mTemporaryDir.path()).filePath("init");
  GitRepository repository = GitRepository::init(directory);
  QVERIFY2(repository.isValid(), qPrintable(repository.errorString()));
  QCOMPARE(repository.workingDirectory(), QFileInfo(directory).canonicalFilePath());
  QVERIFY(QFileInfo(QDir(directory).filePath(".git")).isDir());
  QCOMPARE(git({"rev-parse", "--show-toplevel"}, directory), QFileInfo(directory).canonicalFilePath());
}

void GitRepositoryTest::configAndHash()
{
  initRepository("config");
  git({"config", "core.autocrlf", "true"});
  writeFile("M.mo", "model M\r\nend M;\r\n");
  GitRepository repository = GitRepository::open(mRepositoryPath);
  QVERIFY2(repository.isValid(), qPrintable(repository.errorString()));
  QCOMPARE(repository.configValue("user.name"), QString("OMEdit Test"));
  QCOMPARE(repository.configValue("user.email"), QString("test@example.com"));
  QVERIFY(repository.configValue("omedit.nosuchkey").isEmpty());
  QCOMPARE(repository.fileHash(filePath("M.mo")), std::optional<QString>(git({"hash-object", "M.mo"})));
  QCOMPARE(GitRepository::hashFile(filePath("M.mo")), std::optional<QString>(git({"hash-object", "--no-filters", "M.mo"})));
  QVERIFY(repository.fileHash(filePath("M.mo")) != GitRepository::hashFile(filePath("M.mo")));
}

void GitRepositoryTest::stageAndUnstage()
{
  initRepository("stage");
  writeFile("A.mo", "model A\nend A;\n");
  writeFile("B.mo", "model B\nend B;\n");
  GitRepository repository = GitRepository::open(mRepositoryPath);
  QVERIFY2(repository.isValid(), qPrintable(repository.errorString()));
  // Before the first commit.
  QVERIFY2(repository.stage(filePath("A.mo")), qPrintable(repository.errorString()));
  QCOMPARE(git({"status", "--porcelain", "A.mo"}), QString("A  A.mo"));
  QVERIFY2(repository.unstage(filePath("A.mo")), qPrintable(repository.errorString()));
  QCOMPARE(git({"status", "--porcelain", "A.mo"}), QString("?? A.mo"));

  commitFiles({"A.mo", "B.mo"}, "First");
  writeFile("A.mo", "model A\n  Real x;\nend A;\n");
  QVERIFY2(repository.stage(filePath("A.mo")), qPrintable(repository.errorString()));
  QCOMPARE(git({"status", "--porcelain", "A.mo"}), QString("M  A.mo"));
  QVERIFY2(repository.unstage(filePath("A.mo")), qPrintable(repository.errorString()));
  QCOMPARE(git({"status", "--porcelain", "A.mo"}), QString("M A.mo"));
  QCOMPARE(repository.fileContent(filePath("A.mo")), std::optional<QByteArray>("model A\nend A;\n"));

  QVERIFY(QFile::remove(filePath("B.mo")));
  QVERIFY2(repository.stage(filePath("B.mo")), qPrintable(repository.errorString()));
  QCOMPARE(git({"status", "--porcelain", "B.mo"}), QString("D  B.mo"));

  QVERIFY(!repository.stage(QDir(mTemporaryDir.path()).filePath("outside.mo")));
  QVERIFY(!repository.errorString().isEmpty());
}

void GitRepositoryTest::commit()
{
  initRepository("commit");
  GitRepository repository = GitRepository::open(mRepositoryPath);
  QVERIFY2(repository.isValid(), qPrintable(repository.errorString()));
  QVERIFY(!repository.commit("Nothing"));
  QCOMPARE(repository.errorString(), QString("Nothing to commit."));

  writeFile("A.mo", "model A\nend A;\n");
  QVERIFY(repository.stage(filePath("A.mo")));
  QVERIFY(!repository.commit("  \n"));
  std::optional<QString> first = repository.commit("First commit\n\nWith a body.");
  QVERIFY2(first, qPrintable(repository.errorString()));
  QCOMPARE(*first, git({"rev-parse", "HEAD"}));
  QCOMPARE(git({"log", "-1", "--format=%an <%ae>%n%B"}), QString("OMEdit Test <test@example.com>\nFirst commit\n\nWith a body."));
  QCOMPARE(git({"status", "--porcelain"}), QString());

  QVERIFY(!repository.commit("Again"));
  QCOMPARE(repository.errorString(), QString("Nothing to commit."));

  writeFile("A.mo", "model A\n  Real x;\nend A;\n");
  writeFile("B.mo", "model B\nend B;\n");
  QVERIFY(repository.stage(filePath("A.mo")));
  std::optional<QString> second = repository.commit("Second", "Other Author", "other@example.com");
  QVERIFY2(second, qPrintable(repository.errorString()));
  QCOMPARE(git({"log", "-1", "--format=%an <%ae>|%cn <%ce>|%P"}), QString("Other Author <other@example.com>|OMEdit Test <test@example.com>|") + *first);
  QCOMPARE(git({"status", "--porcelain"}), QString("?? B.mo"));
}

void GitRepositoryTest::commitFile()
{
  initRepository("commitfile");
  writeFile("A.mo", "model A\nend A;\n");
  GitRepository repository = GitRepository::open(mRepositoryPath);
  QVERIFY2(repository.isValid(), qPrintable(repository.errorString()));
  std::optional<QString> first = repository.commitFile(filePath("A.mo"), "Add A");
  QVERIFY2(first, qPrintable(repository.errorString()));
  QCOMPARE(git({"show", "--format=", "--name-status", "HEAD"}), QString("A\tA.mo"));

  // Other staged changes stay staged and are not committed.
  writeFile("A.mo", "model A\n  Real x;\nend A;\n");
  writeFile("B.mo", "model B\nend B;\n");
  git({"add", "B.mo"});
  std::optional<QString> second = repository.commitFile(filePath("A.mo"), "Change A");
  QVERIFY2(second, qPrintable(repository.errorString()));
  QCOMPARE(git({"show", "--format=%s", "--name-status", "HEAD"}), QString("Change A\n\nM\tA.mo"));
  QCOMPARE(git({"status", "--porcelain"}), QString("A  B.mo"));
  QCOMPARE(repository.fileHash(filePath("A.mo")), std::optional<QString>(git({"rev-parse", "HEAD:A.mo"})));

  QVERIFY(!repository.commitFile(filePath("A.mo"), "Unchanged"));
  QCOMPARE(repository.errorString(), QString("Nothing to commit."));

  QVERIFY(QFile::remove(filePath("A.mo")));
  QVERIFY2(repository.commitFile(filePath("A.mo"), "Remove A"), qPrintable(repository.errorString()));
  QCOMPARE(git({"show", "--format=", "--name-status", "HEAD"}), QString("D\tA.mo"));
}

void GitRepositoryTest::logAndShow()
{
  initRepository("log");
  writeFile("A.mo", "model A\nend A;\n");
  commitFiles({"A.mo"}, "Add A");
  writeFile("B.mo", "model B\nend B;\n");
  commitFiles({"B.mo"}, "Add B");
  writeFile("A.mo", "model A\n  Real x;\nend A;\n");
  commitFiles({"A.mo"}, "Change A");

  GitRepository repository = GitRepository::open(mRepositoryPath);
  QVERIFY2(repository.isValid(), qPrintable(repository.errorString()));
  QList<GitRepository::Commit> commits = repository.log();
  QCOMPARE(commits.size(), 3);
  QCOMPARE(commits.at(0).summary, QString("Change A"));
  QCOMPARE(commits.at(0).id, git({"rev-parse", "HEAD"}));
  QVERIFY(commits.at(0).id.startsWith(commits.at(0).shortId));
  QCOMPARE(commits.at(0).authorName, QString("OMEdit Test"));
  QCOMPARE(commits.at(0).time.toSecsSinceEpoch(), git({"log", "-1", "--format=%at"}).toLongLong());
  QCOMPARE(commits.at(2).summary, QString("Add A"));
  QCOMPARE(repository.log(1).size(), 1);
  commits = repository.log(-1, filePath("A.mo"));
  QCOMPARE(commits.size(), 2);
  QCOMPARE(commits.at(0).summary, QString("Change A"));
  QCOMPARE(commits.at(1).summary, QString("Add A"));

  std::optional<QString> show = repository.show("HEAD~1");
  QVERIFY2(show, qPrintable(repository.errorString()));
  QVERIFY(show->startsWith("commit " + git({"rev-parse", "HEAD~1"}) + "\nAuthor: OMEdit Test <test@example.com>\n"));
  QVERIFY(show->contains("\n    Add B\n"));
  QVERIFY(show->contains("B.mo | 2 ++"));
  QVERIFY(show->contains("1 file changed, 2 insertions(+)"));
  QVERIFY(repository.show("HEAD~3") == std::nullopt);
}

void GitRepositoryTest::revert()
{
  initRepository("revert");
  writeFile("A.mo", "model A\nend A;\n");
  writeFile("B.mo", "model B\nend B;\n");
  commitFiles({"A.mo", "B.mo"}, "First");
  writeFile("A.mo", "model A\n  Real x;\nend A;\n");
  commitFiles({"A.mo"}, "Change A");
  const QString changeA = git({"rev-parse", "HEAD"});
  writeFile("B.mo", "model B\n  Real y;\nend B;\n");
  commitFiles({"B.mo"}, "Change B");
  // A local change of a file that the revert does not touch is kept.
  writeFile("B.mo", "model B\n  Real z;\nend B;\n");

  GitRepository repository = GitRepository::open(mRepositoryPath);
  QVERIFY2(repository.isValid(), qPrintable(repository.errorString()));
  std::optional<QString> revertId = repository.revert(changeA);
  QVERIFY2(revertId, qPrintable(repository.errorString()));
  QCOMPARE(*revertId, git({"rev-parse", "HEAD"}));
  QCOMPARE(git({"log", "-1", "--format=%B"}), QString("Revert \"Change A\"\n\nThis reverts commit %1.").arg(changeA));
  QCOMPARE(git({"show", "--format=", "--name-status", "HEAD"}), QString("M\tA.mo"));
  QFile file(filePath("A.mo"));
  QVERIFY(file.open(QIODevice::ReadOnly));
  QCOMPARE(file.readAll(), QByteArray("model A\nend A;\n"));
  QCOMPARE(git({"status", "--porcelain"}), QString("M B.mo"));
}

void GitRepositoryTest::revertConflict()
{
  initRepository("revertconflict");
  writeFile("A.mo", "model A\nend A;\n");
  commitFiles({"A.mo"}, "First");
  writeFile("A.mo", "model A\n  Real x;\nend A;\n");
  commitFiles({"A.mo"}, "Add x");
  const QString addX = git({"rev-parse", "HEAD"});
  writeFile("A.mo", "model A\n  Real x(start = 1);\nend A;\n");
  commitFiles({"A.mo"}, "Start x");
  const QString head = git({"rev-parse", "HEAD"});

  GitRepository repository = GitRepository::open(mRepositoryPath);
  QVERIFY2(repository.isValid(), qPrintable(repository.errorString()));
  // Conflicts with a later commit.
  QVERIFY(!repository.revert(addX));
  QVERIFY(!repository.errorString().isEmpty());
  QCOMPARE(git({"rev-parse", "HEAD"}), head);
  QCOMPARE(git({"status", "--porcelain"}), QString());

  // Would overwrite a local change.
  writeFile("A.mo", "model A\n  Real x(start = 2);\nend A;\n");
  QVERIFY(!repository.revert("HEAD"));
  QVERIFY(!repository.errorString().isEmpty());
  QCOMPARE(git({"rev-parse", "HEAD"}), head);
  QFile file(filePath("A.mo"));
  QVERIFY(file.open(QIODevice::ReadOnly));
  QCOMPARE(file.readAll(), QByteArray("model A\n  Real x(start = 2);\nend A;\n"));
}

QTEST_APPLESS_MAIN(GitRepositoryTest)

#include "GitRepositoryTest.moc"
