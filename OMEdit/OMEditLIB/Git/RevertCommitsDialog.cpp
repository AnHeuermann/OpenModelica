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

#include "RevertCommitsDialog.h"
#include "MainWindow.h"
#include "Modeling/ModelWidgetContainer.h"
#include "Modeling/MessagesWidget.h"
#include "Util/Helper.h"
#include "Util/StringHandler.h"
#include "QPushButton"
#include "QDialogButtonBox"
#include "QLineEdit"
#include "QFileInfo"
#include "QFrame"
#include "QGridLayout"

/*!
 * \class RevertCommitsDialog
 * \brief Creates a dialog that shows to select a git commit to be reverted.
 */
/*!
 * \brief RevertCommitsDialog::RevertCommitsDialog
 * \param pMainWindow - pointer to MainWindow
 */
RevertCommitsDialog::RevertCommitsDialog(QWidget *pParent)
  : QDialog(pParent)
{
  setWindowTitle(QString(Helper::applicationName).append(" - ").append(tr("Revert Commit")));
  setAttribute(Qt::WA_DeleteOnClose);
  resize(500, 400);
  mRepository = GitRepository::open(MainWindow::instance()->getModelWidgetContainer()->getCurrentModelWidget()->getLibraryTreeItem()->getFileName());
  // Working directory
  mpWorkingDirectoryLabel = new Label(tr("Working Directory:"));
  mpWorkingDirectoryTextBox = new QLineEdit;
  mpWorkingDirectoryTextBox->setText(mRepository.workingDirectory());
  connect(mpWorkingDirectoryTextBox, SIGNAL(textChanged(QString)), this, SLOT(workingDirectoryChanged(QString)));
  mpDirectoryBrowseButton = new QPushButton(tr("Browse Directory"));
  connect(mpDirectoryBrowseButton, SIGNAL(clicked()), SLOT(browseWorkingDirectory()));
  // Commit
  mpCommitLabel = new Label(tr("Commit:"));
  mpCommitTextBox = new QLineEdit;
  mpCommitTextBox->setText("HEAD");
  mpCommitTextBox->setFocus();
  mpCommitTextBox->selectAll();
  mpCommitTextBox->setReadOnly(true);
  connect(mpCommitTextBox, SIGNAL(textChanged(QString)), this, SLOT(commitTextChanged(QString)));
  mpCommitBrowseButton = new QPushButton(tr("Browse Commit"));
  connect(mpCommitBrowseButton, SIGNAL(clicked()), SLOT(browseCommitLog()));
  mpCommitDescriptionTextBox = new QPlainTextEdit;
  mpCommitDescriptionTextBox->setReadOnly(true);
  // Create the buttons
  mpRevertButton = new QPushButton(tr("Revert"));
  mpRevertButton->setEnabled(true);
  connect(mpRevertButton, SIGNAL(clicked()), SLOT(revertCommit()));
  mpCancelButton = new QPushButton(Helper::cancel);
  connect(mpCancelButton, SIGNAL(clicked()), SLOT(reject()));
  // create buttons box
  mpButtonBox = new QDialogButtonBox(Qt::Horizontal);
  mpButtonBox->addButton(mpRevertButton, QDialogButtonBox::ActionRole);
  mpButtonBox->addButton(mpCancelButton, QDialogButtonBox::ActionRole);
  // set the layout
  QGridLayout *pMainLayout = new QGridLayout;
  pMainLayout->setAlignment(Qt::AlignTop | Qt::AlignLeft);
  pMainLayout->addWidget(mpWorkingDirectoryLabel, 0, 0);
  pMainLayout->addWidget(mpWorkingDirectoryTextBox, 0, 1);
  pMainLayout->addWidget(mpDirectoryBrowseButton, 0, 2);
  pMainLayout->addWidget(mpCommitLabel, 1, 0);
  pMainLayout->addWidget(mpCommitTextBox, 1, 1);
  pMainLayout->addWidget(mpCommitBrowseButton, 1, 2);
  pMainLayout->addWidget(mpCommitDescriptionTextBox, 2, 0, 1, 3);
  pMainLayout->addWidget(mpButtonBox, 3, 0, 3, 3,  Qt::AlignRight);
  setLayout(pMainLayout);

  getCommitHistory();
}
/*!
  Slot activated when mpCommitTextBox textChanged signal is raised.\n
  */
void RevertCommitsDialog::commitTextChanged(const QString &commit)
{
  mpCommitDescriptionTextBox->setPlainText(mRepository.show(commit).value_or(mRepository.errorString()));
}

void RevertCommitsDialog::getCommitHistory()
{
  mpCommitDescriptionTextBox->setPlainText(mRepository.show(mpCommitTextBox->text()).value_or(mRepository.errorString()));
}

void RevertCommitsDialog::revertCommit()
{
  if (!mRepository.revert(mpCommitTextBox->text())) {
    MessagesWidget::instance()->addGUIMessage(MessageItem(MessageItem::Modelica, tr("Could not revert %1: %2").arg(mpCommitTextBox->text(), mRepository.errorString()),
                                                          Helper::scriptingKind, Helper::errorLevel));
  }
  accept();
}

void RevertCommitsDialog::browseCommitLog()
{
  LogCommitDialog *pLogCommitDialog = new LogCommitDialog(this);
  pLogCommitDialog->exec();
}

void RevertCommitsDialog::workingDirectoryChanged(const QString &workingDirectory)
{
  Q_UNUSED(workingDirectory);
}

void RevertCommitsDialog::browseWorkingDirectory()
{
  QString workingDirectory = StringHandler::getExistingDirectory(this, QString("%1 - %2").arg(Helper::applicationName).arg(Helper::chooseDirectory), NULL);
  if (workingDirectory.isEmpty())
    return;
  mpWorkingDirectoryTextBox->setText(workingDirectory);
}

void RevertCommitsDialog::setCommit(QString commit)
{
  mpCommitTextBox->setText(commit);
}
/*!
 * \class LogCommitDialog
 * \brief Creates a dialog that shows commit log.
 */
/*!
 * \brief LogCommitDialog::LogCommitDialog
 * \param pMainWindow - pointer to MainWindow
 */

enum Columns
{
    Sha1Column,
    SubjectColumn,
    ColumnCount
};

LogCommitDialog::LogCommitDialog(RevertCommitsDialog *pRevertCommitsDialog)
  : QDialog(pRevertCommitsDialog)
{
  setWindowTitle(QString(Helper::applicationName).append(" - ").append(tr("Select Commit")));
  setAttribute(Qt::WA_DeleteOnClose);
  resize(500, 400);
  mpRevertCommitsDialog = pRevertCommitsDialog;
  mpLogModel = new QStandardItemModel(0, ColumnCount, this);
  QStringList headers;
  headers << tr("SHA-1")<< tr("Subject");
  mpLogModel->setHorizontalHeaderLabels(headers);
  // Log commit tree view
  mpLogTreeView = new QTreeView;
  mpLogTreeView->setModel(mpLogModel);
  mpLogTreeView->setMinimumWidth(300);
  mpLogTreeView->setUniformRowHeights(true);
  mpLogTreeView->setRootIsDecorated(false);
  mpLogTreeView->setSelectionBehavior(QAbstractItemView::SelectRows);
  mpLogTreeView->setSelectionMode(QAbstractItemView::SingleSelection);
  // Create the buttons
  mpOkButton = new QPushButton(tr("Ok"));
  mpOkButton->setEnabled(true);
  connect(mpOkButton, SIGNAL(clicked()), SLOT(ok()));
  mpCancelButton = new QPushButton(Helper::cancel);
  connect(mpCancelButton, SIGNAL(clicked()), SLOT(reject()));
  // create buttons box
  mpButtonBox = new QDialogButtonBox(Qt::Horizontal);
  mpButtonBox->addButton(mpOkButton, QDialogButtonBox::ActionRole);
  mpButtonBox->addButton(mpCancelButton, QDialogButtonBox::ActionRole);
  // set the layout
  QGridLayout *pMainLayout = new QGridLayout;
  pMainLayout->setAlignment(Qt::AlignTop | Qt::AlignLeft);
  pMainLayout->addWidget(mpLogTreeView, 0, 0);
  pMainLayout->addWidget(mpButtonBox, 1, 0);
  setLayout(pMainLayout);

  getCommitLog();
}

void LogCommitDialog::getCommitLog()
{
  QString repository = MainWindow::instance()->getModelWidgetContainer()->getCurrentModelWidget()->getLibraryTreeItem()->getFileName();
  const QList<GitRepository::Commit> commits = GitRepository::open(repository).log(100);

  const QString currentCommit = this->commit();
  int selected = currentCommit.isEmpty() ? 0 : -1;
  if (const int rowCount = mpLogModel->rowCount())
      mpLogModel->removeRows(0, rowCount);

  foreach (const GitRepository::Commit &commit, commits) {
    QList<QStandardItem *> row;
    for (int c = 0; c < ColumnCount; ++c) {
      auto item = new QStandardItem;
      item->setFlags(Qt::ItemIsEnabled | Qt::ItemIsSelectable);
      row.push_back(item);
    }
    row[Sha1Column]->setText(commit.shortId);
    row[SubjectColumn]->setText(commit.summary);
    mpLogModel->appendRow(row);
    if (selected == -1 && currentCommit == commit.shortId)
      selected = mpLogModel->rowCount() - 1;
  }
  mpLogTreeView->setCurrentIndex(mpLogModel->index(selected, 0));
}

QString LogCommitDialog::commit() const
{
  if (const QStandardItem *sha1Item = currentItem(Sha1Column))
      return sha1Item->text();
  return QString();
}

const QStandardItem *LogCommitDialog::currentItem(int column) const
{
  const QModelIndex currentIndex = mpLogTreeView->selectionModel()->currentIndex();
  if (currentIndex.isValid())
      return mpLogModel->item(currentIndex.row(), column);
  return 0;
}

int LogCommitDialog::commitIndex() const
{
  const QModelIndex currentIndex = mpLogTreeView->selectionModel()->currentIndex();
  if (currentIndex.isValid())
      return currentIndex.row();
  return -1;
}

void LogCommitDialog::ok()
{
  mpRevertCommitsDialog->setCommit(commit());
  accept();

}
