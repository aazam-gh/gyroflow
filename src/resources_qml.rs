// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright © 2021-2022 Adrian <adrian.eddy at gmail>

use qmetaobject::qrc;

qrc!(pub rsrc_qml,
    "/" {
        "src/ui/main_window.qml",
        "src/ui/App.qml",
        "src/ui/VideoArea.qml",
        "src/ui/Calibrator.qml",
        "src/ui/CalibrationTarget.qml",
        "src/ui/Shortcuts.qml",
        "src/ui/RenderQueue.qml",
        "src/ui/Statistics.qml",
        "src/ui/SettingsSelector.qml",
        "src/ui/Util.js",

        "src/ui/menu/Advanced.qml",
        "src/ui/menu/Export.qml",
        "src/ui/menu/LensProfile.qml",
        "src/ui/menu/LensCalibrate.qml",
        "src/ui/menu/MotionData.qml",
        "src/ui/menu/Stabilization.qml",
        "src/ui/menu/Synchronization.qml",
        "src/ui/menu/VideoInformation.qml",
        "src/ui/components/AdvancedSection.qml",
        "src/ui/components/BasicText.qml",
        "src/ui/components/Button.qml",
        "src/ui/components/CheckBox.qml",
        "src/ui/components/CheckBoxWithContent.qml",
        "src/ui/components/ComboBox.qml",
        "src/ui/components/DropdownChevron.qml",
        "src/ui/components/DropTarget.qml",
        "src/ui/components/DropTargetRect.qml",
        "src/ui/components/Ease.qml",
        "src/ui/components/FileDialog.qml",
        "src/ui/components/FrequencyChart.qml",
        "src/ui/components/Hr.qml",
        "src/ui/components/Label.qml",
        "src/ui/components/LensProfileSearchDelegate.qml",
        "src/ui/components/LinkButton.qml",
        "src/ui/components/LoaderOverlay.qml",
        "src/ui/components/MenuItem.qml",
        "src/ui/components/Modal.qml",
        "src/ui/components/NumberField.qml",
        "src/ui/components/OutputPathField.qml",
        "src/ui/components/Popup.qml",
        "src/ui/components/PopupDelegate.qml",
        "src/ui/components/Menu.qml",
        "src/ui/components/Action.qml",
        "src/ui/components/RadioButton.qml",
        "src/ui/components/ResizablePanel.qml",
        "src/ui/components/SearchField.qml",
        "src/ui/components/SidePanel.qml",
        "src/ui/components/Slider.qml",
        "src/ui/components/SliderWithField.qml",
        "src/ui/components/SplitButton.qml",
        "src/ui/components/TabbedPopup.qml",
        "src/ui/components/TableList.qml",
        "src/ui/components/TextArea.qml",
        "src/ui/components/TextField.qml",
        "src/ui/components/Timeline.qml",
        "src/ui/components/TimelineAxisButton.qml",
        "src/ui/components/TimelineRangeIndicator.qml",
        "src/ui/components/TimelineSyncPoint.qml",
        "src/ui/components/ToolTip.qml",
        "src/ui/components/InfoMessage.qml",
        "src/ui/components/InfoMessageSmall.qml",
        "src/ui/components/ItemLoader.qml",
        "src/ui/components/WindowCloseButton.qml",
    }
);
