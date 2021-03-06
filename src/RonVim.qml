import QtQuick 2.6
import QtQuick.Controls 1.0
import QtQuick.Dialogs 1.2
import QtQuick.Layouts 1.3
import QtQml 2.2

import Ronvim 1.0

ApplicationWindow {
    id: window
    width: 800
    height: 600
    title: "RonVim"
    visible: true

    function quit() {
        console.log("root.quit() called");
         frontend.run_command("quit");
         Qt.quit();
    }

    menuBar: MenuBar {
        id: menu
        Menu {
            title: qsTr("File")
            MenuItem {
                text: qsTr("New File")
                onTriggered: mainSplit.newTab();
            }
            MenuItem {
                text: qsTr("Open File...")
                onTriggered: mainSplit.openTab();
            }
            MenuItem {
                text: qsTr("Save")
                onTriggered: frontend.run_command("save");
            }
            MenuItem {
                text: qsTr("Save As...")
                onTriggered: frontend.run_command("prompt_save_as");
            }
            MenuItem {
                text: qsTr("Save All")
                onTriggered: frontend.run_command("save_all")
            }
            MenuSeparator{}
            MenuItem {
                text: qsTr("New Window")
                onTriggered: frontend.run_command("new_window");
            }
            MenuItem {
                text: qsTr("Close Window")
                onTriggered: frontend.run_command("close_window");
            }
            MenuSeparator{}
            MenuItem {
                text: qsTr("Close File")
                onTriggered: frontend.run_command("close");
            }
            MenuItem {
                text: qsTr("Close All Files")
                onTriggered: frontend.run_command("close_all");
            }
            MenuSeparator{}
            MenuItem {
                text: qsTr("Quit")
               
                onTriggered: window.quit();
            }
        }
        Menu {
            title: qsTr("Find")
            MenuItem {
                text: qsTr("Find Next")
                onTriggered: frontend.run_command("find_next");
            }
        }
        Menu {
            title: qsTr("Edit")
            MenuItem {
                text: qsTr("Undo")
                onTriggered: frontend.run_command("undo");
            }
            MenuItem {
                text: qsTr("Redo")
                onTriggered: frontend.run_command("redo");
            }
            Menu {
                title: qsTr("Undo Selection")
                MenuItem {
                    text: qsTr("Soft Undo")
                    onTriggered: frontend.run_command("soft_undo");
                }
                MenuItem {
                    text: qsTr("Soft Redo")
                    onTriggered: frontend.run_command("soft_redo");
                }
            }
            MenuSeparator{}
            MenuItem {
                text: qsTr("Copy")
                onTriggered: frontend.run_command("copy");
            }
            MenuItem {
                text: qsTr("Cut")
                onTriggered: frontend.run_command("cut");
            }
            MenuItem {
                text: qsTr("Paste")
                onTriggered: frontend.run_command("paste");
            }
        }
        Menu {
            title: qsTr("View")
            MenuItem {
                text: qsTr("Show/Hide Console")
             // TODO   onTriggered: { consoleView.visible = !consoleView.visible }
            }
            MenuItem {
                text: qsTr("Show LeftTab")
                onTriggered: mainSplit.showLeftTab();
            }
        }
    }
    
    SplitView {
        id: mainSplit;
        orientation: Qt.Horizontal;
        anchors.fill: parent;


        Rectangle {
            id: leftTab ;
            Layout.minimumWidth: 20;
            width: 100;
            visible:  false;
            color: "blue";
        }

        TabView {
            id: rightTab;
            Layout.minimumWidth: 20;
            visible:  false;
        }


        function showLeftTab() {
            leftTab.visible = true;
        }

        function newTab() {
            rightTab.visible = true;
            var editor = rightTab.addTab( "newfile *", cmpEditor).children[0]
            editor.editorText.close.connect(closeTab(editor))

        }
        function openTab() {
            rightTab.visible = true;
            var editor = rightTab.addTab("-", cmpEditor).children[0]
            var op = classFileDialog.createObject(window, {callback: editor} );
            op.open();
        }
        function closeTab(tab) {
            console.log("close tab!")
        }
    }

    Component {
        id: classFileDialog
        FileDialog {
            id: fileDialog;
            title: "Choose a file";
            folder: shortcuts.home;
            property var callback;
            onAccepted: {
                callback.fileUrl = fileUrl
            }
        }
    }
    Component {
        id: cmpEditor;
        Editor {}
    }
}
