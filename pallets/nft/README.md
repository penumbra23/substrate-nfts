# NFT pallet

`pallet-nft` extends the FRAME uniques pallet by adding NFT methods on top of it. NFTs are grouped inside collections which share properties (similar image styles, similar real-world items,).

This pallet demonstrates a basic NFT with simple metadata (JSON or IPFS link).

## Functions

- `create_collection(collection_id)` - creates a collection with a specific ID
- `mint(collection_id, item_id, metadata, owner, transferable)` - mints a new NFT inside a collection with the given metadata to a specific address (owner)
- `transfer(collection_id, item_id, destination)` - transfers the NFT from the sender to the destination address

## License
MIT