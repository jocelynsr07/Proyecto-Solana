use anchor_lang::prelude::*;

declare_id!("5ofnf1qYb9n2UmtMPZ9Rri2CCFBbcRk3DDtWi5ganDeg");

#[program]
pub mod nutricionista_program {
    use super::*;

    pub fn registrar_paciente(
        ctx: Context<RegisterPatient>, 
        dni_paciente: String,
        nombre: String, 
        edad: u8,
        estatura_cm: u16,
        es_mujer: bool,
    ) -> Result<()> {
        let paciente = &mut ctx.accounts.patient_account;
        paciente.nutriologa = ctx.accounts.nutriologa.key();
        paciente.dni = dni_paciente;
        paciente.nombre = nombre;
        paciente.edad = edad;
        paciente.estatura_cm = estatura_cm;
        paciente.es_mujer = es_mujer;
        
        Ok(())
    }

    pub fn registrar_evaluacion(
        ctx: Context<UpdatePatient>,
        _dni_paciente: String, // Se usa en la validación de seeds
        peso: u16,           
        _perimetros: [u16; 6], 
        pliegues: [u16; 8],   
    ) -> Result<()> {
        let p = &mut ctx.accounts.patient_account;
        
        p.peso = peso;
        p.p_tricipital = pliegues[1];
        p.p_subescapular = pliegues[2];
        p.p_abdominal = pliegues[3];
        p.p_supraespinal = pliegues[5];
        p.p_muslo = pliegues[6];
        p.p_pantorrilla = pliegues[7];

        // Sumatoria de 6 pliegues (índices ISAK correctos)
        let s6 = p.p_tricipital + p.p_subescapular + p.p_supraespinal + 
                 p.p_abdominal + p.p_muslo + p.p_pantorrilla;
        p.sumatoria_6_pliegues = s6;

        // Cálculo de porcentaje de grasa usando aritmética de enteros (Yuhasz)
        // Multiplicamos por constantes escaladas para preservar precisión sin usar flotantes
        let s6_u32 = s6 as u32;
        if p.es_mujer {
            // Fórmula: (Sumatoria * 0.1548) + 3.58
            p.porcentaje_grasa = ((s6_u32 * 1548) / 10000 + 358) as u16;
        } else {
            // Fórmula: (Sumatoria * 0.1051) + 2.58
            p.porcentaje_grasa = ((s6_u32 * 1051) / 10000 + 258) as u16;
        }

        msg!("Evaluación completa. % Grasa Estimado: {}.{}%", p.porcentaje_grasa / 10, p.porcentaje_grasa % 10);
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct PatientAccount {
    pub nutriologa: Pubkey,
    #[max_len(15)]
    pub dni: String,
    #[max_len(40)]
    pub nombre: String,
    pub edad: u8,
    pub estatura_cm: u16,
    pub es_mujer: bool,
    
    pub peso: u16,
    pub p_tricipital: u16,
    pub p_subescapular: u16,
    pub p_abdominal: u16,
    pub p_supraespinal: u16,
    pub p_muslo: u16,
    pub p_pantorrilla: u16,

    pub sumatoria_6_pliegues: u16,
    pub porcentaje_grasa: u16,
}

#[derive(Accounts)]
#[instruction(dni_paciente: String)] 
pub struct RegisterPatient<'info> {
    #[account(
        init,
        seeds = [b"evaluacion", dni_paciente.as_bytes(), nutriologa.key().as_ref()], 
        bump, 
        payer = nutriologa, 
        space = 8 + PatientAccount::INIT_SPACE
    )]
    pub patient_account: Account<'info, PatientAccount>,
    #[account(mut)]
    pub nutriologa: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(dni_paciente: String)]
pub struct UpdatePatient<'info> {
    #[account(
        mut,
        seeds = [b"evaluacion", dni_paciente.as_bytes(), nutriologa.key().as_ref()],
        bump,
        has_one = nutriologa
    )]
    pub patient_account: Account<'info, PatientAccount>,
    pub nutriologa: Signer<'info>,
}