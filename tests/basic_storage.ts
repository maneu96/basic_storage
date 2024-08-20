import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BasicStorage } from "../target/types/basic_storage";

describe("basic_storage", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BasicStorage as Program<BasicStorage>;

  it("Is initialized!", async () => {
    // Add your test here.
    const seeds= [];
    const [myStorage, _bump ] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);

    console.log("the storage account address is", myStorage.toBase58());

    const tx = await program.methods.initialize()
    .accounts({
      myStorage: myStorage
    })
    .rpc();
  });

  it("Changes coordinates two times", async ()=>{
    await program.methods.changeCoordinates(new anchor.BN(1), new anchor.BN(2))
    .rpc();

    await program.methods.changeCoordinates(new anchor.BN(2), new anchor.BN(1))
    .rpc();
  });
});
