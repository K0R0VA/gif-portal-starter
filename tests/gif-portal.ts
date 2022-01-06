import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { GifPortal } from "../target/types/gif_portal";
import assert from "assert";

type Anchor = {
  workspace: {
    GifPortal: Program<GifPortal>;
  };
};

describe("gif", async () => {
  const link =
    "https://images-ext-2.discordapp.net/external/FU4vqwM_XSMUUHougZ3W1bnCqUJ9bTr75wjeODECbLI/%3Fcid%3D73b8f7b1a70e063e9cf54cc8020e3c15f5fb46f3846597be%26rid%3Dgiphy.mp4%26ct%3Dg/https/media4.giphy.com/media/zzALYeLqMLDa6PEV2C/giphy.mp4";
  const root: Anchor = anchor as unknown as Anchor;
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = root.workspace.GifPortal as Program<GifPortal>;
  const baseAccount = anchor.web3.Keypair.generate();

  it("should-create-account", async () => {
    await program.rpc.createAccount({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [baseAccount],
    });
    const { totalGifs } = await program.account.baseAccount.fetch(
      baseAccount.publicKey
    );
    assert.equal(totalGifs, 0);
  });
  it("should-add-gif", async () => {
    await program.rpc.addGif(link, {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
      },
    });
    const { totalGifs, links }: any = await program.account.baseAccount.fetch(
      baseAccount.publicKey
    );
    const [new_link] = links;
    assert.equal(totalGifs, 1);
    assert.equal(new_link.link, link);
  });
  it("should-upvote", async () => {
    await program.rpc.upvote(link, {
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
      },
    });

    const { links }: any = await program.account.baseAccount.fetch(
      baseAccount.publicKey
    );
    const [upvouted_link] = links;
    assert.equal(1, upvouted_link.totalUpvotes);
  });
  it('shoud-tip', async () => {
    const tx = await program.rpc.tip(new anchor.BN(3), {
      accounts: {
        from: provider.wallet.publicKey,
        gifAuthor: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
    },);
    assert.notEqual(undefined, tx);
  });
});
