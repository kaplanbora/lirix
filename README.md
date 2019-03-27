# Lirix
Get lyrics from [Darklyrics](http://www.darklyrics.com) while listening to music from players like Spotify, Mopidy, MPD etc. 

![Screenshot](https://github.com/kaplanbora/lirix/blob/master/lirix.jpg)

## How to use
Clone the repo, build it and put the binary on your path directory
```
git clone https://github.com/kaplanbora/lirix.git
cd lirix
cargo build --release
cp target/release/lirix ~/.local/bin
```

If you don't want to build it yourself download the binary from release. You should only do this if you don't have rust installed. Building it from the source is always the safest choice.
```
wget https://github.com/kaplanbora/lirix/releases/download/0.1.0/lirix
chmod +x ./lirix
cp ./lirix ~/.local/bin
```

Then just open a terminal and open lirix while listening to music
```
lirix
```

## Supported Players
Lirix supports any player that has a MPRIS interface that uses D-Bus. Most players either support that interface or have plugins/extensions to add that functionality.
