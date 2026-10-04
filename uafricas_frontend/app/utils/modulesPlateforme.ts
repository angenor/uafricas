/**
 * Les vingt modules de la plateforme, tels que le document client les nomme.
 * Un concours s'y rattache par son code (feature 014) : c'est une information
 * d'affichage, pas une règle de jeu, d'où une liste ici et pas une table.
 */
export const MODULES_PLATEFORME: Array<{ code: string, libelle: string }> = [
  { code: 'afrolang', libelle: 'Afrolang' },
  { code: 'codimoi', libelle: 'Codimoi' },
  { code: 'afripulse', libelle: 'Afripulse' },
  { code: 'afroculture', libelle: 'Afroculture' },
  { code: 'rootstree', libelle: 'Rootstree' },
  { code: 'africonnect', libelle: 'Africonnect' },
  { code: 'diapertise', libelle: 'Diapertise' },
  { code: 'sabbafrica', libelle: 'Sabbafrica' },
  { code: 'afromarket', libelle: 'Afromarket' },
  { code: 'factcheck', libelle: 'FactCheck Africa' },
  { code: 'ideaforces', libelle: 'IdeaForces' },
  { code: 'badgoodhabit', libelle: 'BadGoodHabit' },
  { code: 'africalive', libelle: 'AfricaLive' },
  { code: 'humantech', libelle: 'HumanTech' },
  { code: 'numetech', libelle: 'NuMeTech' },
  { code: 'muniversa', libelle: 'Muniversa' },
  { code: 'africantives', libelle: 'Africantives' },
  { code: 'vidafrica', libelle: 'VidAfrica' },
  { code: 'africans_tele', libelle: 'Africans Télé' },
  { code: 'africans_radio', libelle: 'Africans Radio' },
]

export const libelleModulePlateforme = (code: string | null | undefined) =>
  MODULES_PLATEFORME.find(m => m.code === code)?.libelle ?? code ?? ''
