/*
 * This file is part of OpenModelica.
 *
 * Copyright (c) 1998-CurrentYear, Open Source Modelica Consortium (OSMC),
 * c/o Linköpings universitet, Department of Computer and Information Science,
 * SE-58183 Linköping, Sweden.
 *
 * All rights reserved.
 *
 * THIS PROGRAM IS PROVIDED UNDER THE TERMS OF GPL VERSION 3 LICENSE OR 
 * THIS OSMC PUBLIC LICENSE (OSMC-PL) VERSION 1.2. 
 * ANY USE, REPRODUCTION OR DISTRIBUTION OF THIS PROGRAM CONSTITUTES RECIPIENT'S ACCEPTANCE
 * OF THE OSMC PUBLIC LICENSE OR THE GPL VERSION 3, ACCORDING TO RECIPIENTS CHOICE. 
 *
 * The OpenModelica software and the Open Source Modelica
 * Consortium (OSMC) Public License (OSMC-PL) are obtained
 * from OSMC, either from the above address,
 * from the URLs: http://www.ida.liu.se/projects/OpenModelica or  
 * http://www.openmodelica.org, and in the OpenModelica distribution. 
 * GNU version 3 is obtained from: http://www.gnu.org/copyleft/gpl.html.
 *
 * This program is distributed WITHOUT ANY WARRANTY; without
 * even the implied warranty of  MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE, EXCEPT AS EXPRESSLY SET FORTH
 * IN THE BY RECIPIENT SELECTED SUBSIDIARY LICENSE CONDITIONS OF OSMC-PL.
 *
 * See the full OSMC Public License conditions for more details.
 *
 */
/*
 * 
 * @author Adeel Asghar <adeel.asghar@liu.se>
 *
 * RCS: $Id$
 *
 */

#include "VariablesWidget.h"

using namespace OMPlot;

/*!
  \class VariablesTreeItem
  \brief Contains the information about the result variable.
  */
/*!
  \param variableItemData - a list of items.\n
  0 -> filePath\n
  1 -> fileName\n
  2 -> name\n
  3 -> displayName\n
  4 -> value\n
  5 -> description\n
  6 -> tooltip
  */
VariablesTreeItem::VariablesTreeItem(const QVector<QVariant> &variableItemData, VariablesTreeItem *pParent, bool isRootItem)
{
  mpParentVariablesTreeItem = pParent;
  mIsRootItem = isRootItem;
  mFilePath = variableItemData[0].toString();
  mFileName = variableItemData[1].toString();
  mVariableName = variableItemData[2].toString();
  mDisplayVariableName = variableItemData[3].toString();
  mValue = variableItemData[4].toString();
  mDescription = variableItemData[5].toString();
  mToolTip = variableItemData[6].toString();
  mChecked = false;
  mEditable = false;
}

VariablesTreeItem::~VariablesTreeItem()
{
  qDeleteAll(mChildren);
  mChildren.clear();
}

QString VariablesTreeItem::getPlotVariable()
{
  return QString(mVariableName).remove(0, mFileName.length() + 1);
}

QIcon VariablesTreeItem::getVariableTreeItemIcon(QString name) const
{
  if (name.endsWith(".mat"))
    return QIcon(":/Resources/icons/mat.svg");
  else if (name.endsWith(".plt"))
    return QIcon(":/Resources/icons/plt.svg");
  else if (name.endsWith(".csv"))
    return QIcon(":/Resources/icons/csv.svg");
  else
    return QIcon(":/Resources/icons/mat.svg");
}

void VariablesTreeItem::insertChild(int position, VariablesTreeItem *pVariablesTreeItem)
{
  mChildren.insert(position, pVariablesTreeItem);
}

VariablesTreeItem* VariablesTreeItem::child(int row)
{
  return mChildren.value(row);
}

int VariablesTreeItem::childCount() const
{
  return mChildren.count();
}

void VariablesTreeItem::removeChildren()
{
  qDeleteAll(mChildren);
  mChildren.clear();
}

void VariablesTreeItem::removeChild(VariablesTreeItem *pVariablesTreeItem)
{
  mChildren.removeOne(pVariablesTreeItem);
}

int VariablesTreeItem::columnCount() const
{
  return 3;
}

bool VariablesTreeItem::setData(int column, const QVariant &value, int role)
{
  if (column == 0 && role == Qt::CheckStateRole)
  {
    if (value.toInt() == Qt::Checked)
      setChecked(true);
    else if (value.toInt() == Qt::Unchecked)
      setChecked(false);
    return true;
  }
  else if (column == 1 && role == Qt::EditRole)
  {
    mValue = value.toString();
    return true;
  }
  return false;
}

QVariant VariablesTreeItem::data(int column, int role) const
{
  switch (column)
  {
    case 0:
      switch (role)
      {
        case Qt::DisplayRole:
          return mDisplayVariableName;
        case Qt::DecorationRole:
          return mIsRootItem ? getVariableTreeItemIcon(mVariableName) : QIcon();
        case Qt::ToolTipRole:
          return mToolTip;
        case Qt::CheckStateRole:
          if (mChildren.size() > 0)
            return QVariant();
          else
            return isChecked() ? Qt::Checked : Qt::Unchecked;
        default:
          return QVariant();
      }
    case 1:
      switch (role)
      {
        case Qt::DisplayRole:
          return mValue;
        case Qt::EditRole:
          return mValue;
        default:
          return QVariant();
      }
    case 2:
      switch (role)
      {
        case Qt::DisplayRole:
          return mDescription;
        default:
          return QVariant();
      }
    default:
      return QVariant();
  }
}

int VariablesTreeItem::row() const
{
  if (mpParentVariablesTreeItem)
    return mpParentVariablesTreeItem->mChildren.indexOf(const_cast<VariablesTreeItem*>(this));

  return 0;
}

VariablesTreeItem* VariablesTreeItem::parent()
{
  return mpParentVariablesTreeItem;
}

VariablesTreeModel::VariablesTreeModel(VariablesTreeView *pVariablesTreeView)
  : QAbstractItemModel(pVariablesTreeView)
{
  mpVariablesTreeView = pVariablesTreeView;
  QVector<QVariant> headers;
  headers << "" << "" << "Variables" << tr("Variables") << tr("Value") << tr("Description") << "";
  mpRootVariablesTreeItem = new VariablesTreeItem(headers, 0, true);
}

int VariablesTreeModel::columnCount(const QModelIndex &parent) const
{
  if (parent.isValid())
    return static_cast<VariablesTreeItem*>(parent.internalPointer())->columnCount();
  else
    return mpRootVariablesTreeItem->columnCount();
}

int VariablesTreeModel::rowCount(const QModelIndex &parent) const
{
  VariablesTreeItem *pParentVariablesTreeItem;
  if (parent.column() > 0)
    return 0;

  if (!parent.isValid())
    pParentVariablesTreeItem = mpRootVariablesTreeItem;
  else
    pParentVariablesTreeItem = static_cast<VariablesTreeItem*>(parent.internalPointer());
  return pParentVariablesTreeItem->childCount();
}

QVariant VariablesTreeModel::headerData(int section, Qt::Orientation orientation, int role) const
{
  if (orientation == Qt::Horizontal && role == Qt::DisplayRole)
    return mpRootVariablesTreeItem->data(section);
  return QVariant();
}

QModelIndex VariablesTreeModel::index(int row, int column, const QModelIndex &parent) const
{
  if (!hasIndex(row, column, parent))
    return QModelIndex();

  VariablesTreeItem *pParentVariablesTreeItem;

  if (!parent.isValid())
    pParentVariablesTreeItem = mpRootVariablesTreeItem;
  else
    pParentVariablesTreeItem = static_cast<VariablesTreeItem*>(parent.internalPointer());

  VariablesTreeItem *pChildVariablesTreeItem = pParentVariablesTreeItem->child(row);
  if (pChildVariablesTreeItem)
    return createIndex(row, column, pChildVariablesTreeItem);
  else
    return QModelIndex();
}

QModelIndex VariablesTreeModel::parent(const QModelIndex &index) const
{
  if (!index.isValid())
    return QModelIndex();

  VariablesTreeItem *pChildVariablesTreeItem = static_cast<VariablesTreeItem*>(index.internalPointer());
  VariablesTreeItem *pParentVariablesTreeItem = pChildVariablesTreeItem->parent();
  if (pParentVariablesTreeItem == mpRootVariablesTreeItem)
    return QModelIndex();

  return createIndex(pParentVariablesTreeItem->row(), 0, pParentVariablesTreeItem);
}

bool VariablesTreeModel::setData(const QModelIndex &index, const QVariant &value, int role)
{
  VariablesTreeItem *pVariablesTreeItem = getVariablesTreeItem(index);
  bool result = pVariablesTreeItem->setData(index.column(), value, role);
  if (index.column() == 0 && role == Qt::CheckStateRole)
  {
    if (!signalsBlocked())
      emit itemChecked(index);
  }
  emit dataChanged(index, index);
  return result;
}

QVariant VariablesTreeModel::data(const QModelIndex &index, int role) const
{
  if (!index.isValid())
    return QVariant();

  VariablesTreeItem *pVariablesTreeItem = static_cast<VariablesTreeItem*>(index.internalPointer());
  return pVariablesTreeItem->data(index.column(), role);
}

Qt::ItemFlags VariablesTreeModel::flags(const QModelIndex &index) const
{
  if (!index.isValid())
      return 0;

  VariablesTreeItem *pVariablesTreeItem = static_cast<VariablesTreeItem*>(index.internalPointer());
  Qt::ItemFlags flags = Qt::ItemIsEnabled | Qt::ItemIsSelectable;
  if (index.column() == 0 && pVariablesTreeItem && pVariablesTreeItem->childCount() == 0)
    flags |= Qt::ItemIsUserCheckable;
  else if (index.column() == 1 && pVariablesTreeItem && pVariablesTreeItem->childCount() == 0 && pVariablesTreeItem->isEditable())
    flags |= Qt::ItemIsEditable;

  return flags;
}

VariablesTreeItem* VariablesTreeModel::findVariablesTreeItem(const QString &name, VariablesTreeItem *root) const
{
  if (root->getVariableName() == name)
    return root;
  for (int i = root->getChildren().size(); --i >= 0; )
    if (VariablesTreeItem *item = findVariablesTreeItem(name, root->getChildren().at(i)))
      return item;
  return 0;
}

QModelIndex VariablesTreeModel::variablesTreeItemIndex(const VariablesTreeItem *pVariablesTreeItem) const
{
  return VariablesTreeItemIndexHelper(pVariablesTreeItem, mpRootVariablesTreeItem, QModelIndex());
}

QModelIndex VariablesTreeModel::VariablesTreeItemIndexHelper(const VariablesTreeItem *pVariablesTreeItem,
                                                               const VariablesTreeItem *pParentVariablesTreeItem,
                                                               const QModelIndex &parentIndex) const
{
  if (pVariablesTreeItem == pParentVariablesTreeItem)
    return parentIndex;
  for (int i = pParentVariablesTreeItem->getChildren().size(); --i >= 0; ) {
    const VariablesTreeItem *childItem = pParentVariablesTreeItem->getChildren().at(i);
    QModelIndex childIndex = index(i, 0, parentIndex);
    QModelIndex index = VariablesTreeItemIndexHelper(pVariablesTreeItem, childItem, childIndex);
    if (index.isValid())
      return index;
  }
  return QModelIndex();
}

void VariablesTreeModel::insertVariablesItems(QString fileName, QString filePath, QStringList variablesList,
                                              SimulationOptions simulationOptions)
{
  QString toolTip = tr("Simulation Result File: %1\n%2: %3/%4").arg(fileName).arg(Helper::fileLocation).arg(filePath).arg(fileName);
  QRegExp resultTypeRegExp("(_res.mat|_res.plt|_res.csv)");
  QString text = QString(fileName).remove(resultTypeRegExp);
  QModelIndex index = variablesTreeItemIndex(mpRootVariablesTreeItem);
  QVector<QVariant> Variabledata;
  Variabledata << filePath << fileName << fileName << text << "" << "" << toolTip;
  VariablesTreeItem *pTopVariablesTreeItem = new VariablesTreeItem(Variabledata, mpRootVariablesTreeItem, true);
  pTopVariablesTreeItem->setSimulationOptions(simulationOptions);
  int row = rowCount();
  beginInsertRows(index, row, row);
  mpRootVariablesTreeItem->insertChild(row, pTopVariablesTreeItem);
  endInsertRows();
  /* open the model_init.xml file for reading */
  QString initFileName = QString(fileName).replace(resultTypeRegExp, "_init.xml");
  QFile initFile(QString(filePath).append(QDir::separator()).append(initFileName));
  QDomDocument initXmlDocument;
  if (initFile.open(QIODevice::ReadOnly))
  {
    if (!initXmlDocument.setContent(&initFile))
    {
      MessagesWidget *pMessagesWidget = mpVariablesTreeView->getVariablesWidget()->getMainWindow()->getMessagesWidget();
      pMessagesWidget->addGUIMessage(new MessagesTreeItem("", false, 0, 0, 0, 0,
                                                          tr("Unable to set the content of QDomDocument from file %1")
                                                          .arg(initFile.fileName()), Helper::scriptingKind, Helper::errorLevel, 0,
                                                          pMessagesWidget->getMessagesTreeWidget()));
    }
    initFile.close();
  }
  else
  {
    MessagesWidget *pMessagesWidget = mpVariablesTreeView->getVariablesWidget()->getMainWindow()->getMessagesWidget();
    pMessagesWidget->addGUIMessage(new MessagesTreeItem("", false, 0, 0, 0, 0,
                                                        tr("Unable to open the file %1").arg(initFile.fileName()), Helper::scriptingKind,
                                                        Helper::errorLevel, 0, pMessagesWidget->getMessagesTreeWidget()));
  }
  QStringList variables;
  foreach (QString plotVariable, variablesList)
  {
    QString parentVariable;
    if (plotVariable.startsWith("der("))
    {
      QString str = plotVariable;
      str.chop((str.lastIndexOf("der(")/4)+1);
      variables = makeVariableParts(str.mid(str.lastIndexOf("der(") + 4));
    }
    else
    {
      variables = makeVariableParts(plotVariable);
    }
    int count = 1;
    foreach (QString variable, variables)
    {
      QString findVariable = parentVariable.isEmpty() ? fileName + "." + variable : fileName + "." + parentVariable + "." + variable;
      if (findVariablesTreeItem(findVariable, mpRootVariablesTreeItem))
      {
        if (count == 1)
          parentVariable = variable;
        else
          parentVariable += "." + variable;
        count++;
        continue;
      }
      VariablesTreeItem *pParentVariablesTreeItem = findVariablesTreeItem(fileName + "." + parentVariable, mpRootVariablesTreeItem);
      if (!pParentVariablesTreeItem)
      {
        pParentVariablesTreeItem = pTopVariablesTreeItem;
      }
      QModelIndex index = variablesTreeItemIndex(pParentVariablesTreeItem);
      QVector<QVariant> variableData;
      /* if last item */
      if (variables.size() == count && plotVariable.startsWith("der("))
        variableData << filePath << fileName << fileName + "." + plotVariable << "der(" + variable + ")";
      else
        variableData << filePath << fileName << pParentVariablesTreeItem->getVariableName() + "." + variable << variable;
      /* find the variable in the xml file */
      QString variableToFind = variableData[2].toString();
      variableToFind.remove(QRegExp(pTopVariablesTreeItem->getVariableName() + "."));
      /* get the variable value */
      bool found = false;
      variableData << StringHandler::unparse(QString("\"").append(getVariableValue(variableToFind, initXmlDocument, &found)).append("\""));
      /* get the variable description */
      variableData << StringHandler::unparse(QString("\"").append(getVariableDescription(variableToFind, initXmlDocument)).append("\""));
      /* construct tooltip text */
      variableData << tr("File: %1/%2\nVariable: %3").arg(filePath).arg(fileName).arg(variableToFind);
      VariablesTreeItem *pVariablesTreeItem = new VariablesTreeItem(variableData, pParentVariablesTreeItem);
      pVariablesTreeItem->setEditable(found);
      int row = rowCount(index);
      beginInsertRows(index, row, row);
      pParentVariablesTreeItem->insertChild(row, pVariablesTreeItem);
      endInsertRows();
      if (count == 1)
        parentVariable = variable;
      else
        parentVariable += "." + variable;
      count++;
    }
  }
  mpVariablesTreeView->collapseAll();
  QModelIndex idx = variablesTreeItemIndex(pTopVariablesTreeItem);
  idx = mpVariablesTreeView->getVariablesWidget()->getVariableTreeProxyModel()->mapFromSource(idx);
  mpVariablesTreeView->expand(idx);
}

QStringList VariablesTreeModel::makeVariableParts(QString variable)
{
  QStringList variables = variable.split(QRegExp("\\.(?![^\\[\\]]*\\])"), QString::SkipEmptyParts);
  return variables;
}

bool VariablesTreeModel::removeVariableTreeItem(QString variable)
{
  VariablesTreeItem *pVariablesTreeItem = findVariablesTreeItem(variable, mpRootVariablesTreeItem);
  if (pVariablesTreeItem)
  {
    beginRemoveRows(variablesTreeItemIndex(pVariablesTreeItem), 0, pVariablesTreeItem->childCount());
    pVariablesTreeItem->removeChildren();
    VariablesTreeItem *pParentVariablesTreeItem = pVariablesTreeItem->parent();
    pParentVariablesTreeItem->removeChild(pVariablesTreeItem);
    endRemoveRows();
    return true;
  }
  return false;
}

void VariablesTreeModel::unCheckVariables(VariablesTreeItem *pVariablesTreeItem)
{
  QList<VariablesTreeItem*> items = pVariablesTreeItem->getChildren();
  for (int i = 0 ; i < items.size() ; i++)
  {
    items[i]->setData(0, Qt::Unchecked, Qt::CheckStateRole);
    unCheckVariables(items[i]);
  }
}

VariablesTreeItem* VariablesTreeModel::getVariablesTreeItem(const QModelIndex &index) const
{
  if (index.isValid()) {
    VariablesTreeItem *pVariablesTreeItem = static_cast<VariablesTreeItem*>(index.internalPointer());
    if (pVariablesTreeItem)
      return pVariablesTreeItem;
  }
  return mpRootVariablesTreeItem;
}

QString VariablesTreeModel::getVariableValue(QString variableToFind, QDomDocument xmlDocument, bool *found)
{
  if (xmlDocument.isNull())
    return "";
  QDomNodeList variables = xmlDocument.elementsByTagName("ScalarVariable");
  for (int i = 0; i < variables.size(); i++)
  {
    QDomElement element = variables.at(i).toElement();
    if (!element.isNull())
    {
      if (element.attribute("name").compare(variableToFind) == 0)
      {
        QDomElement el = variables.at(i).firstChild().toElement();
        if (!el.isNull())
        {
          if (el.attribute("useStart").compare("true") == 0)
          {
            *found = true;
            return el.attribute("start");
          }
        }
      }
    }
  }
  return "";
}

QString VariablesTreeModel::getVariableDescription(QString variableToFind, QDomDocument xmlDocument)
{
  if (xmlDocument.isNull())
    return "";
  QDomNodeList variables = xmlDocument.elementsByTagName("ScalarVariable");
  for (int i = 0; i < variables.size(); i++)
  {
    QDomElement element = variables.at(i).toElement();
    if (!element.isNull())
    {
      if (element.attribute("name").compare(variableToFind) == 0)
        return element.attribute("description");
    }
  }
  return "";
}

void VariablesTreeModel::removeVariableTreeItem()
{
  QAction *pAction = qobject_cast<QAction*>(sender());
  if (pAction)
  {
    removeVariableTreeItem(pAction->data().toString());
    emit variableTreeItemRemoved(pAction->data().toString());
  }
}

VariableTreeProxyModel::VariableTreeProxyModel(QObject *parent)
  : QSortFilterProxyModel(parent)
{
}

void VariableTreeProxyModel::clearfilter()
{
  invalidateFilter();
}

bool VariableTreeProxyModel::filterAcceptsRow(int sourceRow, const QModelIndex &sourceParent) const
{
  if (!filterRegExp().isEmpty())
  {
    QModelIndex index = sourceModel()->index(sourceRow, 0, sourceParent);
    if (index.isValid())
    {
      // if any of children matches the filter, then current index matches the filter as well
      int rows = sourceModel()->rowCount(index);
      for (int i = 0 ; i < rows ; ++i)
      {
        if (filterAcceptsRow(i, index))
        {
          return true;
        }
      }
      // check current index itself
      VariablesTreeItem *pVariablesTreeItem = static_cast<VariablesTreeItem*>(index.internalPointer());
      if (pVariablesTreeItem)
      {
        QString variableName = pVariablesTreeItem->getVariableName();
        variableName.remove(QRegExp("(_res.mat|_res.plt|_res.csv)"));
        return variableName.contains(filterRegExp());
      }
      else
      {
        return sourceModel()->data(index).toString().contains(filterRegExp());
      }
      QString key = sourceModel()->data(index, filterRole()).toString();
      return key.contains(filterRegExp());
    }
  }
  return QSortFilterProxyModel::filterAcceptsRow(sourceRow, sourceParent);
}

VariablesTreeView::VariablesTreeView(VariablesWidget *pVariablesWidget)
  : QTreeView(pVariablesWidget)
{
  mpVariablesWidget = pVariablesWidget;
  setItemDelegate(new ItemDelegate(this));
  setTextElideMode(Qt::ElideMiddle);
  setIndentation(Helper::treeIndentation);
  setIconSize(Helper::iconSize);
  setContextMenuPolicy(Qt::CustomContextMenu);
  setExpandsOnDoubleClick(false);
  setSortingEnabled(true);
  sortByColumn(0, Qt::AscendingOrder);
}

VariablesWidget::VariablesWidget(MainWindow *pMainWindow)
  : QWidget(pMainWindow)
{
  setMinimumWidth(175);
  mpMainWindow = pMainWindow;
  // create the find text box
  mpFindVariablesTextBox = new QLineEdit(Helper::findVariables);
  mpFindVariablesTextBox->installEventFilter(this);
  connect(mpFindVariablesTextBox, SIGNAL(returnPressed()), SLOT(findVariables()));
  connect(mpFindVariablesTextBox, SIGNAL(textEdited(QString)), SLOT(findVariables()));
  // create the case sensitivity checkbox
  mpFindCaseSensitiveCheckBox = new QCheckBox(tr("Case Sensitive"));
  connect(mpFindCaseSensitiveCheckBox, SIGNAL(toggled(bool)), SLOT(findVariables()));
  // create the find syntax combobox
  mpFindSyntaxComboBox = new QComboBox;
  mpFindSyntaxComboBox->addItem(tr("Regular Expression"), QRegExp::RegExp);
  mpFindSyntaxComboBox->setItemData(0, tr("A rich Perl-like pattern matching syntax."), Qt::ToolTipRole);
  mpFindSyntaxComboBox->addItem(tr("Wildcard"), QRegExp::Wildcard);
  mpFindSyntaxComboBox->setItemData(1, tr("A simple pattern matching syntax similar to that used by shells (command interpreters) for \"file globbing\"."), Qt::ToolTipRole);
  mpFindSyntaxComboBox->addItem(tr("Fixed String"), QRegExp::FixedString);
  mpFindSyntaxComboBox->setItemData(2, tr("Fixed string matching."), Qt::ToolTipRole);
  connect(mpFindSyntaxComboBox, SIGNAL(currentIndexChanged(int)), SLOT(findVariables()));
  // create variables tree widget
  mpVariablesTreeView = new VariablesTreeView(this);
  mpVariablesTreeModel = new VariablesTreeModel(mpVariablesTreeView);
  mpVariableTreeProxyModel = new VariableTreeProxyModel;
  mpVariableTreeProxyModel->setDynamicSortFilter(true);
  mpVariableTreeProxyModel->setSourceModel(mpVariablesTreeModel);
  mpVariablesTreeView->setModel(mpVariableTreeProxyModel);
  mpVariablesTreeView->setColumnWidth(0, 150);
  mpLastActiveSubWindow = 0;
  // create the layout
  QGridLayout *pMainLayout = new QGridLayout;
  pMainLayout->setContentsMargins(0, 0, 0, 0);
  pMainLayout->addWidget(mpFindVariablesTextBox, 0, 0, 1, 2);
  pMainLayout->addWidget(mpFindCaseSensitiveCheckBox, 1, 0);
  pMainLayout->addWidget(mpFindSyntaxComboBox, 1, 1);
  pMainLayout->addWidget(mpVariablesTreeView, 2, 0, 1, 2);
  setLayout(pMainLayout);
  connect(mpVariablesTreeModel, SIGNAL(rowsInserted(QModelIndex,int,int)), mpVariableTreeProxyModel, SLOT(invalidate()));
  connect(mpVariablesTreeModel, SIGNAL(rowsRemoved(QModelIndex,int,int)), mpVariableTreeProxyModel, SLOT(invalidate()));
  connect(mpVariablesTreeModel, SIGNAL(itemChecked(QModelIndex)), SLOT(plotVariables(QModelIndex)));
  connect(mpVariablesTreeView, SIGNAL(customContextMenuRequested(QPoint)), SLOT(showContextMenu(QPoint)));
  connect(pMainWindow->getPlotWindowContainer(), SIGNAL(subWindowActivated(QMdiSubWindow*)), this, SLOT(updateVariablesTree(QMdiSubWindow*)));
  connect(mpVariablesTreeModel, SIGNAL(variableTreeItemRemoved(QString)), pMainWindow->getPlotWindowContainer(), SLOT(updatePlotWindows(QString)));
}

void VariablesWidget::insertVariablesItemsToTree(QString fileName, QString filePath, QStringList variablesList,
                                                 SimulationOptions simulationOptions)
{
  /* Remove the simulation result if we already had it in tree */
  bool variableItemDeleted = mpVariablesTreeModel->removeVariableTreeItem(fileName);
  /* add the plot variables */
  mpVariablesTreeModel->insertVariablesItems(fileName, filePath, variablesList, simulationOptions);
  /* update the plot variables tree */
  if (variableItemDeleted)
    variablesUpdated();
}

void VariablesWidget::variablesUpdated()
{
  foreach (QMdiSubWindow *pSubWindow, mpMainWindow->getPlotWindowContainer()->subWindowList(QMdiArea::StackingOrder))
  {
    PlotWindow *pPlotWindow = qobject_cast<PlotWindow*>(pSubWindow->widget());
    foreach (PlotCurve *pPlotCurve, pPlotWindow->getPlot()->getPlotCurvesList())
    {
      if (pPlotWindow->getPlotType() == PlotWindow::PLOT)
      {
        QString curveNameStructure = QString(pPlotCurve->getFileName()).append(".").append(pPlotCurve->title().text());
        VariablesTreeItem *pVariableTreeItem;
        pVariableTreeItem = mpVariablesTreeModel->findVariablesTreeItem(curveNameStructure, mpVariablesTreeModel->getRootVariablesTreeItem());
        pPlotWindow->getPlot()->removeCurve(pPlotCurve);
        pPlotCurve->detach();
        pPlotWindow->fitInView();
        pPlotWindow->getPlot()->updateLayout();
        if (pVariableTreeItem)
        {
          bool state = mpVariablesTreeModel->blockSignals(true);
          QModelIndex index = mpVariablesTreeModel->variablesTreeItemIndex(pVariableTreeItem);
          mpVariablesTreeModel->setData(index, Qt::Checked, Qt::CheckStateRole);
          plotVariables(index, pPlotWindow);
          mpVariablesTreeModel->blockSignals(state);
        }
      }
      else if (pPlotWindow->getPlotType() == PlotWindow::PLOTPARAMETRIC)
      {
        QString xVariable = QString(pPlotCurve->getFileName()).append(".").append(pPlotCurve->getXVariable());
        VariablesTreeItem *pXVariableTreeItem;
        pXVariableTreeItem = mpVariablesTreeModel->findVariablesTreeItem(xVariable, mpVariablesTreeModel->getRootVariablesTreeItem());
        QString yVariable = QString(pPlotCurve->getFileName()).append(".").append(pPlotCurve->getYVariable());
        VariablesTreeItem *pYVariableTreeItem;
        pYVariableTreeItem = mpVariablesTreeModel->findVariablesTreeItem(yVariable, mpVariablesTreeModel->getRootVariablesTreeItem());
        if (pXVariableTreeItem && pYVariableTreeItem)
        {
          bool state = mpVariablesTreeModel->blockSignals(true);
          QModelIndex xIndex = mpVariablesTreeModel->variablesTreeItemIndex(pXVariableTreeItem);
          mpVariablesTreeModel->setData(xIndex, Qt::Checked, Qt::CheckStateRole);
          plotVariables(xIndex, pPlotWindow);
          QModelIndex yIndex = mpVariablesTreeModel->variablesTreeItemIndex(pYVariableTreeItem);
          mpVariablesTreeModel->setData(yIndex, Qt::Checked, Qt::CheckStateRole);
          plotVariables(yIndex, pPlotWindow);
          mpVariablesTreeModel->blockSignals(state);
        }
        else
        {
          pPlotWindow->getPlot()->removeCurve(pPlotCurve);
          pPlotCurve->detach();
          pPlotWindow->fitInView();
          pPlotWindow->getPlot()->updateLayout();
        }
      }
    }
  }
  updateVariablesTreeHelper(mpMainWindow->getPlotWindowContainer()->currentSubWindow());
}

void VariablesWidget::updateVariablesTreeHelper(QMdiSubWindow *pSubWindow)
{
  if (!pSubWindow)
    return;
  // first clear all the check boxes in the tree
  bool state = mpVariablesTreeModel->blockSignals(true);
  mpVariablesTreeModel->unCheckVariables(mpVariablesTreeModel->getRootVariablesTreeItem());
  mpVariablesTreeModel->blockSignals(state);
  // all plotwindows are closed down then simply return
  if (mpMainWindow->getPlotWindowContainer()->subWindowList().size() == 0)
    return;

  PlotWindow *pPlotWindow = qobject_cast<PlotWindow*>(pSubWindow->widget());
  // now loop through the curves and tick variables in the tree whose curves are on the plot
  state = mpVariablesTreeModel->blockSignals(true);
  foreach (PlotCurve *pPlotCurve, pPlotWindow->getPlot()->getPlotCurvesList())
  {
    VariablesTreeItem *pVariablesTreeItem;
    if (pPlotWindow->getPlotType() == PlotWindow::PLOT)
    {
      QString variable = QString(pPlotCurve->getFileName()).append(".").append(pPlotCurve->title().text());
      pVariablesTreeItem = mpVariablesTreeModel->findVariablesTreeItem(variable, mpVariablesTreeModel->getRootVariablesTreeItem());
      if (pVariablesTreeItem)
        mpVariablesTreeModel->setData(mpVariablesTreeModel->variablesTreeItemIndex(pVariablesTreeItem), Qt::Checked, Qt::CheckStateRole);
    }
    else if (pPlotWindow->getPlotType() == PlotWindow::PLOTPARAMETRIC)
    {
      // check the xvariable
      QString xVariable = QString(pPlotCurve->getFileName()).append(".").append(pPlotCurve->getXVariable());
      pVariablesTreeItem = mpVariablesTreeModel->findVariablesTreeItem(xVariable, mpVariablesTreeModel->getRootVariablesTreeItem());
      if (pVariablesTreeItem)
        mpVariablesTreeModel->setData(mpVariablesTreeModel->variablesTreeItemIndex(pVariablesTreeItem), Qt::Checked, Qt::CheckStateRole);
      // check the y variable
      QString yVariable = QString(pPlotCurve->getFileName()).append(".").append(pPlotCurve->getYVariable());
      pVariablesTreeItem = mpVariablesTreeModel->findVariablesTreeItem(yVariable, mpVariablesTreeModel->getRootVariablesTreeItem());
      if (pVariablesTreeItem)
        mpVariablesTreeModel->setData(mpVariablesTreeModel->variablesTreeItemIndex(pVariablesTreeItem), Qt::Checked, Qt::CheckStateRole);
    }
  }
  mpVariablesTreeModel->blockSignals(state);
  /* call QAbstractItemModel::setData so the treeview is updated. */
  mpVariableTreeProxyModel->invalidate();
  mpVariablesTreeModel->setData(QModelIndex(), QVariant());
}

bool VariablesWidget::eventFilter(QObject *pObject, QEvent *pEvent)
{
  if (pObject != mpFindVariablesTextBox)
    return false;
  if (pEvent->type() == QEvent::FocusIn)
  {
    if (mpFindVariablesTextBox->text().compare(Helper::findVariables) == 0)
      mpFindVariablesTextBox->setText("");
  }
  if (pEvent->type() == QEvent::FocusOut)
  {
    if (mpFindVariablesTextBox->text().isEmpty())
      mpFindVariablesTextBox->setText(Helper::findVariables);
  }
  return false;
}

void VariablesWidget::readVariablesAndUpdateXML(VariablesTreeItem *pVariablesTreeItem, QString outputFileName, QDomDocument xmlDocument)
{
  for (int i = 0 ; i < pVariablesTreeItem->childCount() ; i++)
  {
    VariablesTreeItem *pChildVariablesTreeItem = pVariablesTreeItem->child(i);
    if (pChildVariablesTreeItem->isEditable())
    {
      QString value = pChildVariablesTreeItem->data(1, Qt::DisplayRole).toString();
      QString variableToFind = pChildVariablesTreeItem->getVariableName();
      variableToFind.remove(QRegExp(outputFileName + "."));
      findVariableAndUpdateValue(variableToFind, value, xmlDocument);
    }
    readVariablesAndUpdateXML(pChildVariablesTreeItem, outputFileName, xmlDocument);
  }
}

void VariablesWidget::findVariableAndUpdateValue(QString variableToFind, QString value, QDomDocument xmlDocument)
{
  QDomNodeList variables = xmlDocument.elementsByTagName("ScalarVariable");
  for (int i = 0; i < variables.size(); i++)
  {
    QDomElement element = variables.at(i).toElement();
    if (!element.isNull())
    {
      if (element.attribute("name").compare(variableToFind) == 0)
      {
        QDomElement el = variables.at(i).firstChild().toElement();
        if (!el.isNull())
        {
          el.setAttribute("start", value);
        }
      }
    }
  }
}

void VariablesWidget::plotVariables(const QModelIndex &index, PlotWindow *pPlotWindow)
{
  if (index.column() > 0)
    return;
  VariablesTreeItem *pVariablesTreeItem = static_cast<VariablesTreeItem*>(index.internalPointer());
  if (!pVariablesTreeItem)
    return;
  try
  {
    // if pPlotWindow is 0 then get the current window.
    if (!pPlotWindow)
      pPlotWindow = mpMainWindow->getPlotWindowContainer()->getCurrentWindow();
    // if pPlotWindow is 0 then create a new plot window.
    if (!pPlotWindow)
    {
      mpMainWindow->getPlotWindowContainer()->addPlotWindow();
      pPlotWindow = mpMainWindow->getPlotWindowContainer()->getCurrentWindow();
    }
    // if still pPlotWindow is 0 then return.
    if (!pPlotWindow)
    {
      bool state = mpVariablesTreeModel->blockSignals(true);
      mpVariablesTreeModel->setData(index, Qt::Unchecked, Qt::CheckStateRole);
      mpVariablesTreeModel->blockSignals(state);
      QMessageBox::information(this, QString(Helper::applicationName).append(" - ").append(Helper::information),
                               tr("No plot window is active for plotting. Please select a plot window or open a new."), Helper::ok);
      return;
    }
    // if plottype is PLOT then
    if (pPlotWindow->getPlotType() == PlotWindow::PLOT)
    {
      // check the item checkstate
      if (pVariablesTreeItem->isChecked())
      {
        pPlotWindow->initializeFile(QString(pVariablesTreeItem->getFilePath()).append("/").append(pVariablesTreeItem->getFileName()));
        pPlotWindow->setCurveWidth(mpMainWindow->getOptionsDialog()->getCurveStylePage()->getCurveThickness());
        pPlotWindow->setCurveStyle(mpMainWindow->getOptionsDialog()->getCurveStylePage()->getCurvePattern());
        pPlotWindow->setVariablesList(QStringList(pVariablesTreeItem->getPlotVariable()));
        pPlotWindow->plot();
        pPlotWindow->fitInView();
        pPlotWindow->getPlot()->getPlotZoomer()->setZoomBase(false);
        pPlotWindow->getPlot()->updateLayout();
      }
      // if user unchecks the variable then remove it from the plot
      else if (!pVariablesTreeItem->isChecked())
      {
        foreach (PlotCurve *pPlotCurve, pPlotWindow->getPlot()->getPlotCurvesList())
        {
          QString curveTitle = QString(pPlotCurve->getFileName()).append(".").append(pPlotCurve->title().text());
          if (curveTitle.compare(pVariablesTreeItem->getVariableName()) == 0)
          {
            pPlotWindow->getPlot()->removeCurve(pPlotCurve);
            pPlotCurve->detach();
            pPlotWindow->fitInView();
            pPlotWindow->getPlot()->getPlotZoomer()->setZoomBase(false);
            pPlotWindow->getPlot()->updateLayout();
          }
        }
      }
    }
    // if plottype is PLOTPARAMETRIC then
    else
    {
      // check the item checkstate
      if (pVariablesTreeItem->isChecked())
      {
        // if mPlotParametricVariables is empty just add one QStringlist with 1 varibale to it
        if (mPlotParametricVariables.isEmpty())
        {
          mPlotParametricVariables.append(QStringList(pVariablesTreeItem->getPlotVariable()));
          mFileName = pVariablesTreeItem->getFileName();
        }
        // if mPlotParametricVariables is not empty then add one string to its last element
        else
        {
          if (mPlotParametricVariables.last().size() < 2)
          {
            if (mFileName.compare(pVariablesTreeItem->getFileName()) != 0)
            {
              bool state = mpVariablesTreeModel->blockSignals(true);
              mpVariablesTreeModel->setData(index, Qt::Unchecked, Qt::CheckStateRole);
              QMessageBox::critical(this, QString(Helper::applicationName).append(" - ").append(Helper::error),
                                    GUIMessages::getMessage(GUIMessages::PLOT_PARAMETRIC_DIFF_FILES), Helper::ok);
              mpVariablesTreeModel->blockSignals(state);
              return;
            }
            mPlotParametricVariables.last().append(QStringList(pVariablesTreeItem->getPlotVariable()));
            pPlotWindow->initializeFile(QString(pVariablesTreeItem->getFilePath()).append("/").append(pVariablesTreeItem->getFileName()));
            pPlotWindow->setCurveWidth(mpMainWindow->getOptionsDialog()->getCurveStylePage()->getCurveThickness());
            pPlotWindow->setCurveStyle(mpMainWindow->getOptionsDialog()->getCurveStylePage()->getCurvePattern());
            pPlotWindow->setVariablesList(mPlotParametricVariables.last());
            pPlotWindow->plotParametric();
            if (mPlotParametricVariables.size() > 1)
            {
              pPlotWindow->setXLabel("");
              pPlotWindow->setYLabel("");
            }
            pPlotWindow->fitInView();
            pPlotWindow->getPlot()->getPlotZoomer()->setZoomBase(false);
            pPlotWindow->getPlot()->updateLayout();
          }
          else
          {
            mPlotParametricVariables.append(QStringList(pVariablesTreeItem->getPlotVariable()));
            mFileName = pVariablesTreeItem->getFileName();
          }
        }
      }
      // if user unchecks the variable then remove it from the plot
      else if (!pVariablesTreeItem->isChecked())
      {
        // remove the variable from mPlotParametricVariables list
        foreach (QStringList list, mPlotParametricVariables)
        {
          if (list.contains(pVariablesTreeItem->getPlotVariable()))
          {
            // if list has only one variable then clear the list and return;
            if (list.size() < 2)
            {
              mPlotParametricVariables.removeOne(list);
              break;
            }
            // if list has more than two variables then remove both and remove the curve
            else
            {
              QString itemTitle = QString(list.last()).append("(").append(list.first()).append(")");
              foreach (PlotCurve *pPlotCurve, pPlotWindow->getPlot()->getPlotCurvesList())
              {
                QString curveTitle = pPlotCurve->title().text();
                if ((curveTitle.compare(itemTitle) == 0) && (pVariablesTreeItem->getFileName().compare(pPlotCurve->getFileName()) == 0))
                {
                  bool state = mpVariablesTreeModel->blockSignals(true);
                  // uncheck the x variable
                  QString xVariable = QString(pPlotCurve->getFileName()).append(".").append(pPlotCurve->getXVariable());
                  VariablesTreeItem *pVariablesTreeItem;
                  pVariablesTreeItem = mpVariablesTreeModel->findVariablesTreeItem(xVariable, mpVariablesTreeModel->getRootVariablesTreeItem());
                  if (pVariablesTreeItem)
                    mpVariablesTreeModel->setData(mpVariablesTreeModel->variablesTreeItemIndex(pVariablesTreeItem), Qt::Unchecked, Qt::CheckStateRole);
                  // uncheck the y variable
                  QString yVariable = QString(pPlotCurve->getFileName()).append(".").append(pPlotCurve->getYVariable());
                  pVariablesTreeItem = mpVariablesTreeModel->findVariablesTreeItem(yVariable, mpVariablesTreeModel->getRootVariablesTreeItem());
                  if (pVariablesTreeItem)
                    mpVariablesTreeModel->setData(mpVariablesTreeModel->variablesTreeItemIndex(pVariablesTreeItem), Qt::Unchecked, Qt::CheckStateRole);
                  mpVariablesTreeModel->blockSignals(state);
                  pPlotWindow->getPlot()->removeCurve(pPlotCurve);
                  pPlotCurve->detach();
                  pPlotWindow->fitInView();
                  pPlotWindow->getPlot()->getPlotZoomer()->setZoomBase(false);
                  pPlotWindow->getPlot()->updateLayout();
                }
              }
              mPlotParametricVariables.removeOne(list);
              if (mPlotParametricVariables.size() == 1)
              {
                if (mPlotParametricVariables.last().size() > 1)
                {
                  pPlotWindow->setXLabel(mPlotParametricVariables.last().at(0));
                  pPlotWindow->setYLabel(mPlotParametricVariables.last().at(1));
                }
                else
                {
                  pPlotWindow->setXLabel("");
                  pPlotWindow->setYLabel("");
                }
              }
              else
              {
                pPlotWindow->setXLabel("");
                pPlotWindow->setYLabel("");
              }
            }
          }
        }
      }
    }
  }
  catch (PlotException &e)
  {
    QMessageBox::critical(this, QString(Helper::applicationName).append(" - ").append(Helper::error), e.what(), Helper::ok);
  }
}

void VariablesWidget::updateVariablesTree(QMdiSubWindow *pSubWindow)
{
  if (!pSubWindow && mpMainWindow->getPlotWindowContainer()->subWindowList().size() != 0)
    return;
  /* if the same sub window is activated again then just return */
  if (mpLastActiveSubWindow == pSubWindow)
  {
    mpLastActiveSubWindow = pSubWindow;
    return;
  }
  mpLastActiveSubWindow = pSubWindow;
  updateVariablesTreeHelper(pSubWindow);
}

void VariablesWidget::showContextMenu(QPoint point)
{
  int adjust = 24;
  QModelIndex index = mpVariablesTreeView->indexAt(point);
  index = mpVariableTreeProxyModel->mapToSource(index);
  VariablesTreeItem *pVariablesTreeItem = static_cast<VariablesTreeItem*>(index.internalPointer());
  if (pVariablesTreeItem && pVariablesTreeItem->isRootItem())
  {
    /* delete result action */
    QAction *pDeleteResultAction = new QAction(QIcon(":/Resources/icons/delete.png"), tr("Delete Result"), this);
    pDeleteResultAction->setData(pVariablesTreeItem->getVariableName());
    pDeleteResultAction->setStatusTip(tr("Delete the result"));
    connect(pDeleteResultAction, SIGNAL(triggered()), mpVariablesTreeModel, SLOT(removeVariableTreeItem()));
    /* re-simulate action */
    QAction *pReSimulateAction = new QAction(QIcon(":/Resources/icons/simulate.png"), Helper::simulate, this);
    pReSimulateAction->setData(pVariablesTreeItem->getSimulationOptions());
    pReSimulateAction->setStatusTip(Helper::simulateTip);
    pReSimulateAction->setEnabled(pVariablesTreeItem->getSimulationOptions().isValid());
    connect(pReSimulateAction, SIGNAL(triggered()), this, SLOT(reSimulate()));
    QMenu menu(this);
    menu.addAction(pDeleteResultAction);
    menu.addAction(pReSimulateAction);
    point.setY(point.y() + adjust);
    menu.exec(mpVariablesTreeView->mapToGlobal(point));
  }
}

void VariablesWidget::findVariables()
{
  mpVariableTreeProxyModel->invalidate();
  Qt::CaseSensitivity caseSensitivity = mpFindCaseSensitiveCheckBox->isChecked() ? Qt::CaseSensitive: Qt::CaseInsensitive;
  mpVariableTreeProxyModel->setSortCaseSensitivity(caseSensitivity);
  QString findText = mpFindVariablesTextBox->text();
  if (mpFindVariablesTextBox->text().isEmpty() || (mpFindVariablesTextBox->text().compare(Helper::findVariables) == 0))
  {
    findText = "";
  }
  QRegExp::PatternSyntax syntax = QRegExp::PatternSyntax(mpFindSyntaxComboBox->itemData(mpFindSyntaxComboBox->currentIndex()).toInt());
  QRegExp regExp(findText, caseSensitivity, syntax);
  mpVariableTreeProxyModel->setFilterRegExp(regExp);
}

void VariablesWidget::reSimulate()
{
  QAction *pAction = qobject_cast<QAction*>(sender());
  if (pAction)
  {
    SimulationOptions simulationOptions = pAction->data().value<SimulationOptions>();
    simulationOptions.setReSimuate(true);
    /* Update the _init.xml file with new values. */
    QRegExp resultTypeRegExp("(_res.mat|_res.plt|_res.csv)");
    /* open the model_init.xml file for writing */
    QString initFileName = QString(simulationOptions.getOutputFileName()).replace(resultTypeRegExp, "_init.xml");
    QFile initFile(QString(simulationOptions.getWorkingDirectory()).append(QDir::separator()).append(initFileName));
    QDomDocument initXmlDocument;
    if (initFile.open(QIODevice::ReadOnly))
    {
      if (initXmlDocument.setContent(&initFile))
      {
        VariablesTreeItem *pTopVariableTreeItem;
        pTopVariableTreeItem = mpVariablesTreeModel->findVariablesTreeItem(simulationOptions.getOutputFileName(),
                                                                           mpVariablesTreeModel->getRootVariablesTreeItem());
        if (pTopVariableTreeItem)
        {
          readVariablesAndUpdateXML(pTopVariableTreeItem, simulationOptions.getOutputFileName(), initXmlDocument);
        }
      }
      else
      {
        MessagesWidget *pMessagesWidget = mpVariablesTreeView->getVariablesWidget()->getMainWindow()->getMessagesWidget();
        pMessagesWidget->addGUIMessage(new MessagesTreeItem("", false, 0, 0, 0, 0,
                                                            tr("Unable to set the content of QDomDocument from file %1")
                                                            .arg(initFile.fileName()), Helper::scriptingKind, Helper::errorLevel, 0,
                                                            pMessagesWidget->getMessagesTreeWidget()));
      }
      initFile.close();
      initFile.open(QIODevice::WriteOnly | QIODevice::Truncate);
      QTextStream textStream(&initFile);
      textStream << initXmlDocument.toString();
      initFile.close();
    }
    else
    {
      MessagesWidget *pMessagesWidget = mpVariablesTreeView->getVariablesWidget()->getMainWindow()->getMessagesWidget();
      pMessagesWidget->addGUIMessage(new MessagesTreeItem("", false, 0, 0, 0, 0,
                                                          tr("Unable to open the file %1").arg(initFile.fileName()), Helper::scriptingKind,
                                                          Helper::errorLevel, 0, pMessagesWidget->getMessagesTreeWidget()));
    }
    mpMainWindow->getSimulationDialog()->runSimulationExecutable(simulationOptions);
  }
}
