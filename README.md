# Why this?

We wanted to get uncompressed UEAssets into a Human Readable Format that let us work with it further, so this tool reads uncompressed UEassets into a json representation so that it can be understood

This was only tested with one asset from [Billie Bust Up](https://store.steampowered.com/app/1400830/Billie_Bust_Up/) which is `UE 4.25.4`
# Compiling

Have cargo installed and then

```
git clone https://github.com/RedBrumbler/UEAssetReader.git
cd UEAssetReader
cargo build --release
```

# Downloading

Download the executable through github actions [here](https://github.com/RedBrumbler/UEAssetDumper/actions) for Ubuntu, MacOS and Windows. then you can run it on a command line as such:

```
ue-asset-dumper <FILE_PATH> [-o/--output OUTPUT_PATH]
```
# Credits

 - kaiheilos - [AssetEditor](https://github.com/kaiheilos/Utilities), tool which provided the understanding of the file format for UEassets