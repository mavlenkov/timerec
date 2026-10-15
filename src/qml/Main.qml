import QtQuick
import org.kde.kirigami as Kirigami

Kirigami.ApplicationWindow {
    id: root

    width: 800
    height: 520
    minimumWidth: 360
    minimumHeight: 280
    visible: true
    title: "TimeRec"

    pageStack.initialPage: Kirigami.Page {
        title: qsTr("Главная")

        Kirigami.PlaceholderMessage {
            anchors.centerIn: parent
            width: Math.min(parent.width - Kirigami.Units.largeSpacing * 2, 400)
            icon.source: "qrc:/qt/qml/local/timerec/assets/timerec.svg"
            text: "TimeRec"
            explanation: qsTr("Здесь начинается твоё приложение.")
        }
    }
}
