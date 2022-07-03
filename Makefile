default: rust_ios.ipa

rust_ios:
	cargo build -Z unstable-options --release

rust_ios.app: rust_ios
	mkdir -p rust_ios.app
	cp target/aarch64-apple-ios/release/rust_ios rust_ios.app/app
	cp Info.plist rust_ios.app/
	cp Rust_App_Provisioning_Profile.mobileprovision rust_ios.app/
	codesign -f -s "iPhone Developer: Darby Sauter (56QNC7F8PY)" --entitlements 'rust_ios.entitlements' rust_ios.app

rust_ios.ipa: rust_ios.app
	mkdir -p Payload
	cp -r rust_ios.app Payload
	zip -r rust_ios.ipa Payload
	rm -rf rust_ios.app Payload

.PHONY: clean
clean:
	rm -rf MakeTest.app