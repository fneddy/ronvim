import QtQuick 2.0
import QtQuick.Controls 1.4

import BackendModule 1.0
import Ronvim 1.0

TextEdit {
    id: editorText
    anchors.fill: parent;
    selectByMouse : true;
    focus: true;
    visible: true;
    textFormat : TextEdit.PlainText;


    signal close;
    property url fileUrl

    onFileUrlChanged: {
        title: fileUrl;
        backend.open(fileUrl.toString());
    }

    Keys.onPressed: keypressed(event);
    function keypressed(event) {
       backend.handle_input(event.text, event.key, event.modifiers, event.nativeScanCode);
        event.accepted = true
    }

    RsBackend {
        id: backend;
        cursor_position: editorText.cursorPosition;
        selection_start: editorText.selectionStart;
        selection_end: editorText.selectionEnd;
        text: editorText.text;
    }
    text: backend.text;
    cursorPosition: backend.cursor_position;

    ScrollBar {
        width: 10;
    }

    Text {
        text: "\u2715"
        anchors.bottom: parent.top
        anchors.right: parent.right
        MouseArea {
            anchors.fill: parent;
            onClicked: editorText.close();
        }
    }
 }
