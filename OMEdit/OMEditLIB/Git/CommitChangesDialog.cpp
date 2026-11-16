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

#include "CommitChangesDialog.h"
#include "MainWindow.h"
#include "Modeling/ModelWidgetContainer.h"
#include "Modeling/MessagesWidget.h"
#include "Options/OptionsDialog.h"
#include "Traceability/TraceabilityInformationURI.h"
#include "Util/Helper.h"
#include "QFrame"
#include "QGridLayout"
#include "QGroupBox"
#include "QPushButton"
#include "QDialogButtonBox"
#include "QDateTime"
#include "QTextStream"
#include "QFileInfo"

enum { statusColumn, nameColumn, columnCount};
//enum { fileNameRole = Qt::UserRole, isDirectoryRole = Qt::UserRole + 1 };

/*!
 * \class CommitChangesDialog
 * \brief Creates a dialog that shows the list of unsaved files to be commited.
 */
/*!
 * \brief CommitChangesDialog::CommitChangesDialog
 * \param pMainWindow - pointer to MainWindow
 */
CommitChangesDialog::CommitChangesDialog(QWidget *pParent)
: QDialog(pParent)
{
  setWindowTitle(QString(Helper::applicationName).append(" - ").append(tr("Commit")));
  setAttribute(Qt::WA_DeleteOnClose);
  resize(850, 600);
  mRepository = GitRepository::open(MainWindow::instance()->getModelWidgetContainer()->getCurrentModelWidget()->getLibraryTreeItem()->getFileName());
  // repository information
  mpRepositoryLabel = new Label(tr("Repository:"));
  mpRepositoryNameTextBox = new QLineEdit;
  mpRepositoryNameTextBox->setEnabled(false);
  mpRepositoryNameTextBox->setText(mRepository.workingDirectory());
  //Branch
  mpBranchLabel = new Label(tr("Branch:"));
  mpBranchNameTextBox = new QLineEdit;
  mpBranchNameTextBox->setEnabled(false);
  mpBranchNameTextBox->setText(mRepository.branchName());
  QGroupBox *pRepositoryInformationGroupBox = new QGroupBox(tr("Repository Information:"));
  QGridLayout *pRepositoryInformationGridLayout = new QGridLayout;
  pRepositoryInformationGridLayout->addWidget(mpRepositoryLabel, 0, 0);
  pRepositoryInformationGridLayout->addWidget(mpRepositoryNameTextBox, 0, 1);
  pRepositoryInformationGridLayout->addWidget(mpBranchLabel, 1, 0);
  pRepositoryInformationGridLayout->addWidget(mpBranchNameTextBox, 1, 1);
  pRepositoryInformationGroupBox->setLayout(pRepositoryInformationGridLayout);
  // commit information
  QGroupBox *pCommitInformationGroupBox = new QGroupBox(tr("Commit Information:"));
  mpAuthorLabel = new Label(tr("Author:"));
  mpAuthorTextBox = new QLineEdit;
  mpAuthorTextBox->setText(mRepository.configValue("user.name"));
  mpEmailLabel = new Label(tr("Email:"));
  mpEmailTextBox = new QLineEdit;
  mpEmailTextBox->setText(mRepository.configValue("user.email"));
  QGridLayout *pCommitInformationGridLayout = new QGridLayout;
  pCommitInformationGridLayout->addWidget(mpAuthorLabel, 0, 0);
  pCommitInformationGridLayout->addWidget(mpAuthorTextBox, 0, 1);
  pCommitInformationGridLayout->addWidget(mpEmailLabel, 1, 0);
  pCommitInformationGridLayout->addWidget(mpEmailTextBox, 1, 1);
  pCommitInformationGroupBox->setLayout(pCommitInformationGridLayout);
  // Commit description
  QGroupBox *pCommitDescriptionGroupBox = new QGroupBox(tr("Description:"));
  mpCommitDescriptionTextBox = new QPlainTextEdit;
  QGridLayout *pCommitDescriptionLayout = new QGridLayout;
  pCommitDescriptionLayout->addWidget(mpCommitDescriptionTextBox);
  pCommitDescriptionGroupBox->setLayout(pCommitDescriptionLayout);
  // Select all check box
  mpSelectAllCheckBox = new QCheckBox(tr("Select All"));
  mpCommitChangedFilesModel = new QStandardItemModel(0, columnCount, this);
  QStringList headers;
  headers << tr("Status")<< tr("File");
  mpCommitChangedFilesModel->setHorizontalHeaderLabels(headers);
  // files tree view
  mpCommitChangedFilesTreeView = new QTreeView;
  mpCommitChangedFilesTreeView->setModel(mpCommitChangedFilesModel);
  mpCommitChangedFilesTreeView->setUniformRowHeights(true);
  mpCommitChangedFilesTreeView->setSelectionMode(QAbstractItemView::NoSelection);
  mpCommitChangedFilesTreeView->setAllColumnsShowFocus(true);
  mpCommitChangedFilesTreeView->setRootIsDecorated(false);
  // files layout
  QGroupBox *pCommitFilesGroupBox = new QGroupBox(tr("Files:"));
  QGridLayout *pCommitFilesLayout = new QGridLayout;
  pCommitFilesLayout->addWidget(mpSelectAllCheckBox, 0, 0);
  pCommitFilesLayout->addWidget(mpCommitChangedFilesTreeView, 1, 0);
  pCommitFilesGroupBox->setLayout(pCommitFilesLayout);
  // Create the buttons
  mpCommitButton = new QPushButton(tr("commit"));
  mpCommitButton->setEnabled(false);
  connect(mpCommitButton, SIGNAL(clicked()), SLOT(commitFiles()));
  mpCancelButton = new QPushButton(Helper::cancel);
  connect(mpCancelButton, SIGNAL(clicked()), SLOT(reject()));
  // create buttons box
  mpButtonBox = new QDialogButtonBox(Qt::Horizontal);
  mpButtonBox->addButton(mpCommitButton, QDialogButtonBox::ActionRole);
  mpButtonBox->addButton(mpCancelButton, QDialogButtonBox::ActionRole);
  connect(mpCommitDescriptionTextBox, SIGNAL(textChanged()), this, SLOT(commitDescriptionTextChanged()));

  // set the layout
  QGridLayout *pMainLayout = new QGridLayout;
  pMainLayout->setAlignment(Qt::AlignTop | Qt::AlignLeft);
  pMainLayout->addWidget(pRepositoryInformationGroupBox, 0, 0, 1, 2);
  pMainLayout->addWidget(pCommitInformationGroupBox, 1, 0);
  pMainLayout->addWidget(pCommitDescriptionGroupBox, 2, 0);
  pMainLayout->addWidget(pCommitFilesGroupBox, 3, 0);
  pMainLayout->addWidget(mpButtonBox, 4, 0, Qt::AlignRight);
  setLayout(pMainLayout);

  getChangedFiles();
}

void CommitChangesDialog::getChangedFiles()
{
  QString fileName = MainWindow::instance()->getModelWidgetContainer()->getCurrentModelWidget()->getLibraryTreeItem()->getFileName();

  std::optional<GitRepository::Status> status = mRepository.fileStatus(fileName);

  if (const int oldRowCount = mpCommitChangedFilesModel->rowCount())
      mpCommitChangedFilesModel->removeRows(0, oldRowCount);
  if (status && *status != GitRepository::Current && !status->testFlag(GitRepository::Ignored)) {
    addFile(getFileStatus(*status), QDir(mRepository.workingDirectory()).relativeFilePath(fileName), true);
  }
  for (int c = 0; c < mpCommitChangedFilesModel->columnCount(); c++)
      mpCommitChangedFilesTreeView->resizeColumnToContents(c);
   mpSelectAllCheckBox->setChecked(true);
}

void CommitChangesDialog::addFile(QString fileStatus, QString fileName, bool checked)
{
  Q_UNUSED(checked);
  /*QStyle *style = QApplication::style();
  const QIcon folderIcon = style->standardIcon(QStyle::SP_DirIcon);
  const QIcon fileIcon = style->standardIcon(QStyle::SP_FileIcon);
  const QChar slash = QLatin1Char('/');
  // Clean the trailing slash of directories
//  if (fileName.endsWith(slash))
//      fileName.chop(1);
//  QFileInfo fi(workingDirectory + slash + fileName);
//  bool isDir = fi.isDir();
//  if (isDir)
//      checked = false;
  auto nameItem = new QStandardItem(QDir::toNativeSeparators(fileName));
  nameItem->setFlags(Qt::ItemIsUserCheckable|Qt::ItemIsEnabled);
  nameItem->setIcon(isDir ? folderIcon : fileIcon);
  nameItem->setCheckable(true);
  nameItem->setCheckState(checked ? Qt::Checked : Qt::Unchecked);
  nameItem->setData(QVariant(fi.absoluteFilePath()), fileNameRole);
  mpCleanFilesModel->appendRow(nameItem);*/

   QList<QStandardItem *> row;
   for (int c = 0; c < columnCount; ++c) {
       auto item = new QStandardItem;
       item->setFlags(Qt::ItemIsEnabled | Qt::ItemIsSelectable);
       row.push_back(item);
   }
   row[statusColumn]->setText(fileStatus);
   row[nameColumn]->setText(fileName);
   row[statusColumn]->setData(Qt::Checked, Qt::CheckStateRole);
   row[statusColumn]->setCheckable(true);
   mpCommitChangedFilesModel->appendRow(row);
}

QString CommitChangesDialog::getFileStatus(GitRepository::Status status)
{
  if (status & (GitRepository::IndexNew | GitRepository::WorkTreeNew))
    return "modelCreation";
  else if (status & (GitRepository::IndexDeleted | GitRepository::WorkTreeDeleted))
    return "Deleted";
  else if (status & (GitRepository::IndexRenamed | GitRepository::WorkTreeRenamed))
    return "Renamed";
  else if (status & (GitRepository::IndexModified | GitRepository::WorkTreeModified | GitRepository::IndexTypeChange | GitRepository::WorkTreeTypeChange))
    return "modelModification";
  else
    return "UnknownFileStatus";
}

namespace {

void reportGitError(const QString &message)
{
  MessagesWidget::instance()->addGUIMessage(MessageItem(MessageItem::Modelica, message, Helper::scriptingKind, Helper::errorLevel));
}

QString fileHash(const QString &fileName)
{
  GitRepository repository = GitRepository::open(fileName);
  std::optional<QString> hash = repository.isValid() ? repository.fileHash(fileName) : GitRepository::hashFile(fileName);
  return hash.value_or(QString());
}

QString commitAndGetFileHash(const QString &fileName, const QString &message)
{
  GitRepository repository = GitRepository::open(fileName);
  if (!repository.isValid() || !repository.commitFile(fileName, message)) {
    reportGitError(QObject::tr("Could not commit %1: %2").arg(fileName, repository.errorString()));
  }
  return fileHash(fileName);
}

} // namespace

/*!
  Slot activated when mpCommitDescriptionTextBox textChanged signal is raised.\n
  */
void CommitChangesDialog::commitDescriptionTextChanged()
{
  if(!mpCommitDescriptionTextBox->toPlainText().isEmpty())
     mpCommitButton->setEnabled(true);
  else
     mpCommitButton->setEnabled(false);
}

void CommitChangesDialog::commitFiles()
{
  if (!mRepository.commit(mpCommitDescriptionTextBox->toPlainText(), mpAuthorTextBox->text(), mpEmailTextBox->text())) {
    reportGitError(tr("Could not commit to %1: %2").arg(mRepository.workingDirectory(), mRepository.errorString()));
  }
  accept();
}

void CommitChangesDialog::generateTraceabilityURI(QString activity, QString modelFileName, QString nameStructure, QString fmuFileName)
{
  Q_UNUSED(nameStructure);
  QString toolURI, activityURI, agentURI, sourceModelFileNameURI, fmuFileNameURI, entityType, path, gitHash;
  QDir dir(OptionsDialog::instance()->getTraceabilityPage()->getGitRepository()->text());
  QDateTime time = QDateTime::currentDateTime();
  if(activity.compare("modelDescriptionImport")== 0) {
    entityType = "modelDescriptionFile";
    path = dir.relativeFilePath(modelFileName);
    gitHash = commitAndGetFileHash(modelFileName, activity);
    fmuFileNameURI = "Entity.modelFile:" + path + "#" + gitHash ;
    sourceModelFileNameURI = "Entity.modelDescriptionFile:" + dir.relativeFilePath(fmuFileName) + "#" + fileHash(fmuFileName);
  }else if(activity.compare("fmuImport")== 0) {
    entityType = "fmu";
    path = dir.relativeFilePath(modelFileName);
    gitHash = commitAndGetFileHash(modelFileName, activity);
    fmuFileNameURI = "Entity.modelFile:" + path + "#" + gitHash ;
    sourceModelFileNameURI = "Entity.fmu:" + dir.relativeFilePath(fmuFileName) + "#" + fileHash(fmuFileName);
  }else {
    entityType = "fmu";
    path = dir.relativeFilePath(fmuFileName);
    gitHash = commitAndGetFileHash(fmuFileName, activity);
    fmuFileNameURI = "Entity.fmu:" + path + "#" + gitHash;
    sourceModelFileNameURI = "Entity.modelFile:" + dir.relativeFilePath(modelFileName) + "#" + fileHash(modelFileName);
  }
  toolURI = "Entity.softwareTool:OpenModelica: " + MainWindow::instance()->getOMCProxy()->getVersion();
  agentURI = "Agent:" + OptionsDialog::instance()->getTraceabilityPage()->getEmail()->text();
  activityURI = "Activity."+ activity +":" + time.toTimeZone(QTimeZone::utc()).toString(Qt::ISODate)+ "#" + QUuid::createUuid().toString().mid(1, 36);
  MainWindow::instance()->getTraceabilityInformationURI()->translateURIToJsonMessageFormat(activity,  toolURI,  activityURI,  agentURI,  sourceModelFileNameURI,  fmuFileNameURI, entityType, path, gitHash);
}

void CommitChangesDialog::commitAndGenerateTraceabilityURI(QString fileName)
{
  std::optional<GitRepository::Status> status = GitRepository::open(fileName).fileStatus(fileName);
  QString activity = getFileStatus(status.value_or(GitRepository::Status(GitRepository::Current)));
  QString commitMessage = "OpenModelica Modeling";
  commitMessage = QInputDialog::getMultiLineText(MainWindow::instance(), tr("Commit Message "), tr("Commit description:"));
  QString toolURI, activityURI, agentURI, sourceModelFileNameURI, fmuFileNameURI, gitHash, path;
  QDir dir(OptionsDialog::instance()->getTraceabilityPage()->getGitRepository()->text());
  QDateTime time = QDateTime::currentDateTime();
  toolURI = "Entity.softwareTool:OpenModelica: " + MainWindow::instance()->getOMCProxy()->getVersion();
  agentURI = "Agent:" + OptionsDialog::instance()->getTraceabilityPage()->getEmail()->text();
  activityURI = "Activity."+ activity +":" + time.toTimeZone(QTimeZone::utc()).toString(Qt::ISODate)+ "#" + QUuid::createUuid().toString().mid(1, 36);
  path = dir.relativeFilePath(fileName);
  if(activity.compare("modelModification")== 0) {
    gitHash = fileHash(fileName);
    sourceModelFileNameURI = "Entity.modelFile:" + path + "#" + gitHash;
    fmuFileNameURI = "Entity.modelFile:" + path + "#" + commitAndGetFileHash(fileName, commitMessage);
    MainWindow::instance()->getTraceabilityInformationURI()->translateURIToJsonMessageFormat(activity,  toolURI,  activityURI,  agentURI,  sourceModelFileNameURI,  fmuFileNameURI, "modelFile", path, gitHash);
  }
  else if(activity.compare("modelCreation")== 0) {
    gitHash = commitAndGetFileHash(fileName, commitMessage);
    sourceModelFileNameURI = "Entity.modelFile:" + path + "#" + gitHash;
    MainWindow::instance()->getTraceabilityInformationURI()->translateModelCreationURIToJsonMessageFormat(activity,  toolURI,  activityURI,  agentURI,  sourceModelFileNameURI, "modelFile", path, gitHash);
  }
}


