// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.28;

// Uncomment this line to use console.log
// import "hardhat/console.sol";
import {ERC1155} from "@openzeppelin/contracts/token/ERC1155/ERC1155.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

contract IncheonContents is ERC1155, Ownable {
    string private _name;
    string private _symbol;

    // Mapping to track minted content per address
    mapping(address => mapping(uint256 => bool)) private _mintedContent;

    constructor(address owner, string memory name_, string memory symbol_, string memory uri_) ERC1155(uri_) Ownable(owner) {
        _name = name_;
        _symbol = symbol_;
    }

    function name() public view virtual returns (string memory) {
        return _name;
    }

    function symbol() public view virtual returns (string memory) {
        return _symbol;
    }

    function setName(string memory name_) external onlyOwner {
        _name = name_;
    }

    function setSymbol(string memory symbol_) external onlyOwner {
        _symbol = symbol_;
    }

    function setURI(string memory newuri) external onlyOwner {
        _setURI(newuri);
    }

    function mint(address to, uint256 id, uint256 value) external onlyOwner {
        require(to != owner(), "Creators cannot mint their own content");
        require(!_mintedContent[to][id], "Content already minted by this address");
        _mint(to, id, value, new bytes(0));
        _mintedContent[to][id] = true;
    }

    function mintBatch(address to, uint256[] memory ids, uint256[] memory values) external onlyOwner {
        require(to != owner(), "Creators cannot mint their own content");
        for (uint256 i = 0; i < ids.length; i++) {
            require(!_mintedContent[to][ids[i]], "Content already minted by this address");
            _mintedContent[to][ids[i]] = true;
        }
        _mintBatch(to, ids, values, new bytes(0));
    }
}
