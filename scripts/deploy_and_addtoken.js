// We require the Hardhat Runtime Environment explicitly here. This is optional 
// but useful for running the script in a standalone fashion through `node <script>`.
//
// When running the script with `hardhat run <script>` you'll find the Hardhat
// Runtime Environment's members available in the global scope.
const hre = require("hardhat");

async function main() {
  // Hardhat always runs the compile task when running scripts with its command
  // line interface.
  //
  // If this script is run directly using `node` you may want to call compile 
  // manually to make sure everything is compiled
  // await hre.run('compile');

  // We get the contract to deploy
  const fluidexFactory = await ethers.getContractFactory("Fluidex");
  fluidex = await fluidexFactory.deploy();
  await fluidex.deployed();
  await fluidex.initialize();
  console.log("Fluidex deployed to:", fluidex.address);

  // [sender, acc2] = await ethers.getSigners();
  // senderAddr = sender.address;
  [account] = await ethers.getSigners();
  accountAddr = account.address;
  const erc20Factory = await ethers.getContractFactory("MockERC20");
  const decimal = 2;
  const initialBalance = 10000;
  erc20Mock = await erc20Factory.deploy(
    "Test Token",
    "TST",
    decimal,
    accountAddr,
    initialBalance
  );
  await erc20Mock.deployed();
  console.log("erc20Mock deployed to:", erc20Mock.address);

  await fluidex.addToken(erc20Mock.address);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main()
  .then(() => process.exit(0))
  .catch(error => {
    console.error(error);
    process.exit(1);
  });
