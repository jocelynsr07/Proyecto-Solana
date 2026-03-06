import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NutricionistaProgram } from "../target/types/nutricionista_program";
import { expect } from "chai";
import type { NutricionistaProgram } from "../target/types/nutricionista_program";

describe("nutricionista_program", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NutricionistaProgram as anchor.Program<NutricionistaProgram>;
  
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.NutricionistaProgram as Program<NutricionistaProgram>;
  const nutriologa = provider.wallet.publicKey;

  // Datos de prueba (usando la normalización que acordamos)
  const dniPaciente = "0001";
  
  // Derivar la PDA igual que en Rust
  const [patientAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("evaluacion"),
      Buffer.from(dniPaciente),
      nutriologa.toBuffer()
    ],
    program.programId
  );

  it("Registra un nuevo paciente!", async () => {
    await program.methods
      .registrarPaciente(dniPaciente, "Jocelyn Saucedo Rubalcava", 27, 1700, true)
      .accounts({
        patientAccount: patientAccount,
        nutriologa: nutriologa,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const account = await program.account.patientAccount.fetch(patientAccount);
    expect(account.nombre).to.equal("Jocelyn Saucedo Rubalcava");
    expect(account.estaturaCm).to.equal(1700);
  });

  it("Registra una evaluacion!", async () => {
    const peso = 710; // 71.0 kg
    const perimetros = [985, 535, 0, 0, 0, 0];
    const pliegues = [95, 75, 50, 55, 80, 100, 150, 150];

    await program.methods
      .registrarEvaluacion(dniPaciente, peso, perimetros, pliegues)
      .accounts({
        patientAccount: patientAccount,
        nutriologa: nutriologa,
      })
      .rpc();

    const account = await program.account.patientAccount.fetch(patientAccount);
    expect(account.peso).to.equal(710);
    expect(account.porcentajeGrasa).to.be.greaterThan(0);
    console.log("Porcentaje de grasa calculado:", account.porcentajeGrasa / 10, "%");
  });
});