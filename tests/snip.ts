import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Snip } from "../target/types/snip";
import * as assert from "assert";
const { SystemProgram } = anchor.web3;

describe("snip", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Snip as Program<Snip>;
  const baseAccount = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    let tx = await program.rpc.initialize({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [baseAccount],
    });
    console.log("Your transaction signature", tx);

    // Get the account.
    const account = await program.account.baseAccount.fetch(
      baseAccount.publicKey
    );

    // Check that values are initialized
    assert.deepEqual(account.totalSnips.toString(), "0");
    assert.deepEqual(account.snipsList, []);
  });

  it("Is updated!", async () => {
    let inputCode = "let x = 1;";
    let inputLang = { rust: {} };

    await program.rpc.addSnip(inputCode, inputLang, {
      accounts: {
        baseAccount: baseAccount.publicKey,
      },
    });
    // Get the account.
    const account = await program.account.baseAccount.fetch(
      baseAccount.publicKey
    );

    let objInput = {
      code: inputCode,
      lang: { rust: {} },
      userAddress: baseAccount.publicKey,
    };

    assert.deepEqual(account.totalSnips.toString(), "1");
    assert.deepEqual(account.snipsList, [objInput]);
  });
});
