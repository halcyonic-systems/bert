APP_FOLDER="build/mac"
rm -rf ${APP_FOLDER}
mkdir -p ${APP_FOLDER}
# set the name of the Mac App
APP_NAME="DeepSystemsAnalysis"
# set the name of your rust crate
RUST_CRATE_NAME="bert"
# create the folder structure
mkdir -p "${APP_FOLDER}/${APP_NAME}.app/Contents/MacOS"
mkdir -p "${APP_FOLDER}/${APP_NAME}.app/Contents/Resources"
# copy Info.plist
cp deploy/mac/Info.plist "${APP_FOLDER}/${APP_NAME}.app/Contents/Info.plist"
deploy/mac/gen_icon.sh
# copy the icon (assuming you already have it in Apple ICNS format)
cp deploy/mac/AppIcon.icns "${APP_FOLDER}/${APP_NAME}.app/Contents/Resources/AppIcon.icns"
# copy your Bevy game assets
cp -a assets "${APP_FOLDER}/${APP_NAME}.app/Contents/MacOS/"
# compile the executables for each architecture
cargo build --release --target x86_64-apple-darwin # build for Intel
cargo build --release --target aarch64-apple-darwin # build for Apple Silicon
# combine the executables into a single file and put it in the bundle
lipo "target/x86_64-apple-darwin/release/${RUST_CRATE_NAME}" \
     "target/aarch64-apple-darwin/release/${RUST_CRATE_NAME}" \
     -create -output "${APP_FOLDER}/${APP_NAME}.app/Contents/MacOS/${APP_NAME}"
# create a DMG
rm "${APP_NAME}.dmg"
create-dmg \
  --volname "Deep Systems Analysis" \
  --volicon "deploy/mac/AppIcon.icns" \
  --background "deploy/mac/dmg_bg.png" \
  --window-size 800 400 \
  --icon-size 128 \
  --icon "${APP_NAME}.app" 200 190 \
  --hide-extension "${APP_NAME}.app" \
  --app-drop-link 600 190 \
  "${APP_NAME}.dmg" \
  "${APP_FOLDER}"