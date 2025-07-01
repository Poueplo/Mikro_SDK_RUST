import os
import sys
import json
import sqlite3
from PyQt6.QtWidgets import (
    QApplication, QWidget, QVBoxLayout, QHBoxLayout, QLabel, QComboBox,
    QLineEdit, QPushButton, QFileDialog, QScrollArea, QFormLayout, QMessageBox, QCompleter
)
from PyQt6.QtCore import Qt

class MCUConfigurator(QWidget):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("Embedded Rust System Parameters")
        self.setMinimumSize(800, 600)

        self.current_data = None
        self.field_widgets = {}

        self.database = sqlite3.connect("project_setup\database\database_mikro_sdk_rust.db")
        self.db_cursor = self.database.cursor()
        
        self.init_ui()


    def init_ui(self):
        layout = QVBoxLayout()

        # MCU selection layout
        mcu_layout = QHBoxLayout()
        mcu_label = QLabel("MCU:")
        # Get MCU list
        self.db_cursor.execute("SELECT NAME FROM MCU")
        mcu_list = [row[0] for row in self.db_cursor.fetchall()]
        self.mcu_combo = QComboBox()
        self.mcu_combo.setEditable(True)
        self.mcu_combo.addItems(mcu_list)

        # Enable filtering while typing
        completer = QCompleter(mcu_list, self.mcu_combo)
        completer.setCaseSensitivity(Qt.CaseSensitivity.CaseInsensitive)
        self.mcu_combo.setCompleter(completer)

        # Optional: Hook up a callback
        self.mcu_combo.currentTextChanged.connect(self.load_mcu_config)

        mcu_layout.addWidget(mcu_label)
        mcu_layout.addWidget(self.mcu_combo)
        layout.addLayout(mcu_layout)

        self.setLayout(layout)

        # Clock input
        clock_layout = QHBoxLayout()
        clock_label = QLabel("Clock (MHz):")
        self.clock_input = QLineEdit()
        clock_layout.addWidget(clock_label)
        clock_layout.addWidget(self.clock_input)
        layout.addLayout(clock_layout)

        # Scrollable field section
        self.scroll_area = QScrollArea()
        self.form_widget = QWidget()
        self.form_layout = QFormLayout()
        self.form_widget.setLayout(self.form_layout)
        self.scroll_area.setWidgetResizable(True)
        self.scroll_area.setWidget(self.form_widget)
        layout.addWidget(self.scroll_area)

        # Save button
        self.save_button = QPushButton("Save System Parameters")
        self.save_button.clicked.connect(self.save_parameters)
        layout.addWidget(self.save_button)
    
    def load_mcu_config(self, mcu_name):
        self.field_widgets.clear()
        selected_mcu = self.mcu_combo.currentText()
        self.db_cursor.execute(f"SELECT FAMILY.VENDOR FROM MCU JOIN FAMILY ON MCU.FAMILY = FAMILY.NAME WHERE MCU.NAME = '{selected_mcu}'")
        query_result = self.db_cursor.fetchall()
        if not query_result:
            return
        query_data = query_result[0]
        vendor = query_data[0]
        
        # Clear previous content
        for i in reversed(range(self.form_layout.count())):
            widget = self.form_layout.itemAt(i).widget()
            if widget:
                widget.setParent(None)

        filepath = f"project_setup/core_packages/mcu_definitions/{vendor}/{selected_mcu}.json"
        if not os.path.exists(filepath):
            return

        with open(filepath, "r") as f:
            self.current_data = json.load(f)

        # Set clock value
        self.clock_input.setText(str(self.current_data.get("clock", 0)))

        # Populate visible fields
        for register in self.current_data.get("config_registers", []):
            reg_key = register["key"]
            address = register["address"]
            combined_key = f"{reg_key}|{address}"

            for field in register["fields"]:
                field_key = field["key"]
                label = field.get("label", field_key)
                init_value = field.get("init", None)
                mask = field["mask"]

                if field.get("hidden", False):
                    self.field_widgets.setdefault(combined_key, []).append((mask, init_value))
                    continue

                combo = QComboBox()
                selected_index = -1

                if "settings" in field:
                    for i, setting in enumerate(field["settings"]):
                        combo.addItem(setting["label"], setting["value"])
                        if init_value.lower() == setting["value"].lower():
                            selected_index = i
                
                elif "settings_array" in field:
                    settings = field["settings_array"]
                    min_val = int(settings["min_value"])
                    max_val = int(settings["max_value"])
                    inverted = settings.get("inverted", False)
                    values = list(range(min_val, max_val + 1))
                    if inverted:
                        values = list(reversed(values))

                    for i, val in enumerate(values):
                        val_hex = hex(val)[2:].zfill(8).upper()
                        combo.addItem(f"{field_key} = {val}", val_hex)
                        if init_value == val_hex:
                            selected_index = i
                
                if selected_index != -1:
                    combo.setCurrentIndex(selected_index)

                self.form_layout.addRow(QLabel(label), combo)
                self.field_widgets.setdefault(combined_key, []).append((mask, combo))
    
    def save_parameters(self):
        selected_mcu = self.mcu_combo.currentText()
        self.db_cursor.execute(f"SELECT FAMILY.* FROM MCU JOIN FAMILY ON MCU.FAMILY = FAMILY.NAME WHERE MCU.NAME = '{selected_mcu}'")
        query_result = self.db_cursor.fetchall()[0]
        family_path = query_result[1]
        vendor = query_result[2]
        target = query_result[3]
        query_result_path_index = 3
        current_data = None
        family_template = None
        hal_ll_template = None
        with open(f"project_setup/core_packages/mcu_definitions/{vendor}/{selected_mcu}.json", "r") as f:
            current_data = json.load(f)
        with open(f"family_definitions/{vendor}/{family_path}/Cargo_family_template.toml", "r") as f:
            family_template = f.read()
        with open("hal_ll/hal_ll_Cargo_template.toml", "r") as f:
            hal_ll_template = f.read()

        for language_impl in current_data.get("language_list", []):
            if language_impl.get("language") == "RUST":
                current_data = language_impl


        for module in current_data.get("module_list", []):
            module_name = module.get("module_name")
            sub_module_list = ""
            for sub_module in module.get("sub_modules", []):
                sub_module_name = sub_module.get("sub_module_name")
                pin_mapping = ""
                for pin_feature in sub_module.get("pin_map_features"):
                    pin_mapping += "\""
                    pin_mapping += pin_feature
                    pin_mapping += "\","
                pin_mapping = pin_mapping.rstrip(',')
                if pin_mapping != "":
                    sub_module_list += "\""
                    sub_module_list += sub_module_name
                    sub_module_list += "\","
                family_template = family_template.replace(f"{{{sub_module_name}_features}}", pin_mapping)
            sub_module_list = sub_module_list.rstrip(',')
            hal_ll_template = hal_ll_template.replace(f"{{{module_name}}}", sub_module_list)
            query_result_path_index += 1
        
        hal_ll_template = hal_ll_template.replace(f"{{family}}", family_path)

        with open(f"family_definitions/{vendor}/{family_path}/Cargo.toml", "w") as f:
            f.write(family_template)
        with open("hal_ll/Cargo.toml", "w") as f:
            f.write(hal_ll_template)
        
        with open(f"project_setup/core_packages/memory/{vendor}/{selected_mcu}/memory.x", "r") as f:
            memory_x_contents = f.read()
        with open("memory.x", "w") as f:
            f.write(memory_x_contents)

        with open(f"project_setup/core_packages/mcu_headers/{vendor}/{selected_mcu}/lib.rs", "r") as f:
            mcu_header_contents = f.read()
        with open("core/mcu_header/src/lib.rs", "w") as f:
            f.write(mcu_header_contents)

        with open(f"project_setup/core_packages/startup/{vendor}/{selected_mcu.lower()}.s", "r") as f:
            startup_contents = f.read()
        with open("core/system_reset/src/startup_assembly.s", "w") as f:
            f.write(startup_contents)

        with open(f".cargo/template_config.toml", "r") as f:
            config_contents = f.read()
        config_contents = config_contents.replace(f"{{compiling_target}}", target)
        with open(".cargo/config.toml", "w") as f:
            f.write(config_contents)

        core_header_output = []
        clock = self.clock_input.text()
        selected_mcu = self.mcu_combo.currentText()
        try:
            clock_int = int(clock)
        except ValueError:
            QMessageBox.critical(self, "Error", "Clock must be a valid integer")
            return

        for reg_key, fields in self.field_widgets.items():
            reg_name, addr = reg_key.split("|")
            reg_value = 0
            for mask, widget_or_value in fields:
                if isinstance(widget_or_value, QComboBox):
                    value_hex = widget_or_value.currentData()
                    if value_hex is None:
                        continue
                else:
                    # Hidden field with static init value
                    value_hex = widget_or_value

                value_int = int(value_hex, 16)
                reg_value |= value_int

            core_header_output.append(f"pub const ADDRESS_{reg_name}\t: u32 = 0x{addr};")
            core_header_output.append(f"pub const VALUE_{reg_name}\t: u32 = 0x{reg_value:08X};")

        core_header_output.append(f"pub const FOSC_KHZ_VALUE : u32 = {clock_int * 1000};")
        with open("core/system/src/core_header.rs", "w") as f:
            f.write("\n".join(core_header_output))

        self.db_cursor.execute(f"SELECT SYSTEM_LIB FROM MCU WHERE NAME = '{selected_mcu}'")
        query_result = self.db_cursor.fetchall()[0]
        system_lib = query_result[0]

        with open(f"core/system/system_implementations/{vendor}/{system_lib}.rs", "r") as f:
            system_contents = f.read()
        with open("core/system/src/lib.rs", "w") as f:
            f.write(system_contents)



# Run Application
if __name__ == '__main__':
    app = QApplication(sys.argv)
    win = MCUConfigurator()
    win.show()
    sys.exit(app.exec())