These header files were retrieved using


```
cd /tmp/pdraw

# Install repo
mkdir .bin
curl -sSL https://storage.googleapis.com/git-repo-downloads/repo > .bin/repo
chmod a+x ./.bin/repo
export PATH="$INSTALL_DIR/.bin:$PATH"

# initialise to specific version - only tag that exists currently
repo init -u https://github.com/Parrot-Developers/groundsdk-tools-manifest -b refs/tags/v7.5.0 < $(which yes) || true
repo sync

# Grab the headers
cd $THIS_DIR

cp -r /tmp/pdraw/packages/pdraw/libpdraw/include/pdraw/ pdraw
cp -r /tmp/pdraw/packages/pdraw/libpdraw-vsink/include/pdraw-vsink/ pdraw_vsink

#  cp -r /tmp/pdraw/packages/eigen/include  eigen
cp -r /tmp/pdraw/packages/libfutils/include  libfutils
cp -r /tmp/pdraw/packages/libh264/include  libh264
cp -r /tmp/pdraw/packages/libh265/include  libh265
cp -r /tmp/pdraw/packages/libmedia-buffers/include  libmedia-buffers
#  cp -r /tmp/pdraw/packages/libmedia-buffers-memory/include  libmedia-buffers-memory
#  cp -r /tmp/pdraw/packages/libmedia-buffers-memory-generic/include  libmedia-buffers-memory-generic
cp -r /tmp/pdraw/packages/libmp4/include  libmp4
cp -r /tmp/pdraw/packages/libpomp/include  libpomp
cp -r /tmp/pdraw/packages/librtp/include  librtp
cp -r /tmp/pdraw/packages/librtsp/include  librtsp
cp -r /tmp/pdraw/packages/libsdp/include  libsdp
cp -r /tmp/pdraw/packages/libtransport-packet/include  libtransport-packet
cp -r /tmp/pdraw/packages/libtransport-socket/include  libtransport-socket
#  cp -r /tmp/pdraw/packages/libulog/include  libulog
cp -r /tmp/pdraw/packages/libvideo-decode/include  libvideo-decode
cp -r /tmp/pdraw/packages/libvideo-defs/include  libvideo-defs
cp -r /tmp/pdraw/packages/libvideo-encode/include  libvideo-encode
cp -r /tmp/pdraw/packages/libvideo-encode/core/include  libvideo-encode/core
#cp -r /tmp/pdraw/packages/libvideo-encode-core/include  libvideo-encode-core
cp -r /tmp/pdraw/packages/libvideo-metadata/include  libvideo-metadata
cp -r /tmp/pdraw/packages/libvideo-scale/include  libvideo-scale
cp -r /tmp/pdraw/packages/libvideo-streaming/include  libvideo-streaming
```
