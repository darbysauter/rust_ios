TARGET_GUI       = RustIos
PACKAGE          = com.darby.rustios
VERSION          = 1.0.0

RES              = res
APP 			 = rust_ios.app
ID				 = iPhone Developer: Darby Sauter (56QNC7F8PY)

default: rust_ios.ipa

rust_ios:
	cargo build -Z unstable-options --release

$(APP): rust_ios $(RES)/Info.plist
	mkdir -p $(APP)

	cp target/aarch64-apple-ios/release/rust_ios $(APP)/$(TARGET_GUI)

	cp res/*.mobileprovision $(APP)/embedded.mobileprovision

	sed 's/$$(TARGET)/$(TARGET_GUI)/g;s/$$(PACKAGE)/$(PACKAGE)/g;s/$$(VERSION)/$(VERSION)/g' $(RES)/Info.plist > $(APP)/Info.plist

	cp $(RES)/Icon* $(APP)/

	echo '<?xml version="1.0" encoding="UTF-8"?>' >tmp.plist
	echo '<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">' >>tmp.plist
	echo '<plist version="1.0">' >>tmp.plist
	echo '<dict>' >>tmp.plist
	strings res/*.mobileprovision | egrep -A1 'application-identifier' >>tmp.plist
	strings res/*.mobileprovision | egrep -A1 'team-identifier' >>tmp.plist			
	echo '<key>get-task-allow</key>' >> tmp.plist
	echo '<true/>' >> tmp.plist
	echo '</dict>' >>tmp.plist
	echo '</plist>' >>tmp.plist
	codesign -f -s '$(ID)' --entitlements tmp.plist $(APP)
	rm tmp.plist

rust_ios.ipa: $(APP)
	mkdir -p Payload
	cp -r $(APP) Payload
	rm -rf $(APP)
	zip -r rust_ios.ipa Payload
	rm -rf Payload

install: rust_ios.ipa
	ideviceinstaller -i rust_ios.ipa

.PHONY: clean install
clean:
	rm -rf MakeTest.ipa