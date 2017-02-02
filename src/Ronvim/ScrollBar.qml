import QtQuick 2.0


Rectangle {
    SystemPalette { id: colorPalette; colorGroup: SystemPalette.Active }
    id: scrollbar;
    anchors.right: parent.right;
    anchors.top: parent.top;
    anchors.bottom: parent.bottom;
    width: 10;
    color: colorPalette.window;
    opacity: 0.1;
    border.color: colorPalette.dark;


    MouseArea {
        id: mouser
        anchors.fill: parent;
        hoverEnabled: true;
        onEntered: parent.opacity = 1;
        onExited: parent.opacity = 0.1;
    }

    Rectangle {
        width: parent.width;
        color: colorPalette.highlight;
        border.color: colorPalette.dark;
        height: parent.height / 20;
        MouseArea {
            id: dragger
            anchors.fill: parent;
            drag.target: parent;
            drag.axis: Drag.YAxis;
            drag.minimumY: 0;
            drag.maximumY: scrollbar.height - height;
        }
    }
}

