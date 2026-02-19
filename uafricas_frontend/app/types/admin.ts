// Types TypeScript communs pour l'administration

// ── Reponses API ────────────────────────────────────────────

/** Reponse paginee generique du backend */
export interface PaginatedResponse<T> {
  data: T[]
  total: number
  page: number
  par_page: number
  total_pages: number
}

/** Reponse API standardisee */
export interface ApiResponse<T> {
  success: boolean
  data: T | null
  error: string | null
}

// ── Admin Me ────────────────────────────────────────────────

/** Permission admin */
export interface AdminPermission {
  slug: string
  type_ressource: string
  action: string
}

/** Reponse GET /api/admin/me */
export interface AdminMeResponse {
  id: string
  nom: string
  prenom: string
  email: string
  roles: string[]
  permissions: AdminPermission[]
}

// ── Table ───────────────────────────────────────────────────

/** Definition d'une colonne de tableau */
export interface TableColumn {
  /** Cle correspondant au champ de l'objet */
  key: string
  /** Libelle affiche dans l'en-tete */
  label: string
  /** Colonne triable */
  sortable?: boolean
  /** Largeur CSS (ex: 'w-40', 'w-1/4') */
  width?: string
  /** Alignement du contenu */
  align?: 'left' | 'center' | 'right'
  /** Fonction de formatage personnalisee */
  format?: (value: any, row: any) => string
}

// ── Filtres ─────────────────────────────────────────────────

/** Types de filtres disponibles */
export type FilterType = 'text' | 'select' | 'date' | 'date-range'

/** Option pour un filtre de type select */
export interface FilterSelectOption {
  label: string
  value: string
}

/** Definition d'un filtre */
export interface FilterDefinition {
  /** Cle du parametre de requete */
  key: string
  /** Libelle affiche */
  label: string
  /** Type de filtre */
  type: FilterType
  /** Placeholder du champ */
  placeholder?: string
  /** Options pour les filtres select */
  options?: FilterSelectOption[]
  /** Valeur par defaut */
  defaultValue?: string
}

// ── Pagination ──────────────────────────────────────────────

/** Etat de pagination */
export interface PaginationState {
  page: number
  parPage: number
  total: number
  totalPages: number
}

/** Parametres de tri */
export interface SortState {
  column: string
  direction: 'asc' | 'desc'
}

// ── Stats ───────────────────────────────────────────────────

/** Donnees pour une carte KPI */
export interface StatsCardData {
  label: string
  value: string | number
  icon: string
  /** Couleur du badge/icone (classes Tailwind) */
  color?: string
  /** Variation par rapport a la periode precedente */
  variation?: {
    valeur: number
    type: 'hausse' | 'baisse' | 'stable'
  }
}

// ── Statuts ─────────────────────────────────────────────────

/** Mapping des statuts vers des couleurs daisyUI */
export type BadgeVariant = 'success' | 'warning' | 'error' | 'info' | 'neutral'

export interface StatusConfig {
  label: string
  variant: BadgeVariant
}

/** Configuration des statuts communs */
export const STATUTS: Record<string, StatusConfig> = {
  actif: { label: 'Actif', variant: 'success' },
  en_attente: { label: 'En attente', variant: 'warning' },
  suspendu: { label: 'Suspendu', variant: 'error' },
  bloque: { label: 'Bloque', variant: 'error' },
  supprime: { label: 'Supprime', variant: 'neutral' },
  brouillon: { label: 'Brouillon', variant: 'info' },
  publie: { label: 'Publie', variant: 'success' },
  archive: { label: 'Archive', variant: 'neutral' },
  approuve: { label: 'Approuve', variant: 'success' },
  rejete: { label: 'Rejete', variant: 'error' },
}

// ── Utilisateurs admin ─────────────────────────────────────────

export interface AdminUtilisateur {
  id: string
  nom: string
  prenom: string
  email: string
  photo_url: string | null
  etat: string
  email_verifie: boolean
  derniere_connexion: string | null
  roles: string[]
  created_at: string
}

export interface RoleInfo {
  id: string
  nom: string
  slug: string
  attribue_par_nom: string | null
  created_at: string
}

export interface SpecialiteInfo {
  id: string
  nom: string
  slug: string
}

export interface PermissionSpecifiqueInfo {
  id: string
  permission_slug: string
  type_ressource: string
  action: string
  ressource_type: string
  ressource_id: string
  expire_at: string | null
  created_at: string
}

export interface OrganisationMinimalInfo {
  id: string
  denomination: string
}

export interface AdminUtilisateurDetail {
  id: string
  nom: string
  prenom: string
  email: string
  slug: string | null
  telephone: string | null
  photo_url: string | null
  genre: string
  date_naissance: string | null
  fonction: string | null
  localite: string | null
  ville: string | null
  pays_origine: string | null
  pays_residence: string | null
  organisation: OrganisationMinimalInfo | null
  biographie: string | null
  etat: string
  email_verifie: boolean
  telephone_verifie: boolean
  double_facteur_active: boolean
  documents_verifie: boolean
  bibliotheque_humain: boolean
  langue_preferee: string
  derniere_connexion: string | null
  roles: RoleInfo[]
  specialites: SpecialiteInfo[]
  permissions_specifiques: PermissionSpecifiqueInfo[]
  created_at: string
  updated_at: string
}

export interface CreerUtilisateurForm {
  nom: string
  prenom: string
  email: string
  mot_de_passe: string
  telephone: string
  genre: string
  role_id: string
}

export interface ModifierUtilisateurForm {
  nom?: string
  prenom?: string
  email?: string
  telephone?: string
  genre?: string
  date_naissance?: string
  fonction?: string
  localite?: string
  ville?: string
  pays_origine_id?: string
  pays_residence_id?: string
  organisation_id?: string
  biographie?: string
  langue_preferee?: string
  bibliotheque_humain?: boolean
}

// ── Organisations admin ────────────────────────────────────────

export interface AdminOrganisation {
  id: string
  denomination: string
  type_organisation: string | null
  pays_nom: string | null
  etat: string
  ville: string | null
  nombre_membres: number
  created_at: string
}

export interface AdminOrganisationDetail {
  id: string
  denomination: string
  slug: string | null
  type_organisation: string | null
  pays_id: string | null
  pays_nom: string | null
  email: string | null
  telephone: string | null
  adresse: string | null
  ville: string | null
  site_web: string | null
  logo_url: string | null
  description: string | null
  document_legal_url: string | null
  numero_registre: string | null
  etat: string
  cree_par: string | null
  cree_par_nom: string | null
  nombre_membres: number
  created_at: string
  updated_at: string
}

export interface CreerOrganisationForm {
  denomination: string
  type_organisation: string
  pays_id: string
  email: string
  telephone: string
  adresse: string
  ville: string
  site_web: string
  description: string
  numero_registre: string
}

// ── Partenariats admin ─────────────────────────────────────────

export interface AdminPartenariat {
  id: string
  organisation_id: string
  organisation_denomination: string
  type_partenariat: string | null
  date_debut: string | null
  date_fin: string | null
  actif: boolean
  created_at: string
}

export interface AdminPartenariatDetail {
  id: string
  organisation_id: string
  organisation_denomination: string
  type_partenariat: string | null
  description: string | null
  date_debut: string | null
  date_fin: string | null
  actif: boolean
  approuve_par: string | null
  approuve_par_nom: string | null
  created_at: string
  updated_at: string
}

export interface CreerPartenariatForm {
  organisation_id: string
  type_partenariat: string
  description: string
  date_debut: string
  date_fin: string
}

// ── Roles & Permissions admin ──────────────────────────────────

export interface AdminRole {
  id: string
  nom: string
  slug: string
  description: string | null
  est_systeme: boolean
  nombre_utilisateurs: number
  created_at: string
}

export interface PermissionItem {
  id: string
  nom: string
  slug: string
  description: string | null
  type_ressource: string
  action: string
}

export interface AdminRoleDetail {
  id: string
  nom: string
  slug: string
  description: string | null
  est_systeme: boolean
  permissions: PermissionItem[]
  nombre_utilisateurs: number
  created_at: string
  updated_at: string
}

export interface CreerRoleForm {
  nom: string
  description: string
}

export interface PermissionListeItem {
  id: string
  nom: string
  slug: string
  description: string | null
  type_ressource: string
  action: string
  created_at: string
}

// ── Referentiels — Pays ──────────────────────────────────────

export interface AdminPays {
  id: string
  nom: string
  code_iso2: string | null
  code_iso3: string | null
  capitale: string | null
  continent: string | null
  actif: boolean
  created_at: string
}

export interface AdminPaysDetail {
  id: string
  nom: string
  code_iso2: string | null
  code_iso3: string | null
  indicatif_tel: string | null
  capitale: string | null
  continent: string | null
  longitude: number | null
  latitude: number | null
  actif: boolean
  created_at: string
  updated_at: string | null
}

export interface CreerPaysForm {
  nom: string
  code_iso2: string
  code_iso3: string
  indicatif_tel: string
  capitale: string
  continent: string
  longitude: number | null
  latitude: number | null
}

// ── Referentiels — Domaines & Secteurs ───────────────────────

export interface AdminDomaine {
  id: string
  nom: string
  slug: string
  icone: string | null
  actif: boolean
  created_at: string
}

export interface AdminDomaineDetail {
  id: string
  nom: string
  slug: string
  description: string | null
  icone: string | null
  actif: boolean
  created_at: string
  updated_at: string | null
}

export interface CreerDomaineForm {
  nom: string
  description: string
  icone: string
}

// ── Referentiels — Categories ────────────────────────────────

export interface AdminCategorie {
  id: string
  nom: string
  slug: string
  contexte: string | null
  parent_id: string | null
  icone: string | null
  ordre: number | null
  actif: boolean
  created_at: string
}

export interface AdminCategorieEnfant {
  id: string
  nom: string
  slug: string
  icone: string | null
  ordre: number | null
  actif: boolean
}

export interface AdminCategorieDetail {
  id: string
  nom: string
  slug: string
  contexte: string | null
  parent_id: string | null
  description: string | null
  icone: string | null
  ordre: number | null
  actif: boolean
  created_at: string
  updated_at: string | null
  enfants: AdminCategorieEnfant[]
}

export interface CreerCategorieForm {
  nom: string
  contexte: string
  parent_id: string
  description: string
  icone: string
  ordre: number
}

// ── Referentiels — Tags ──────────────────────────────────────

export interface AdminTag {
  id: string
  nom: string
  slug: string
  created_at: string
}

export interface AdminTagDetail {
  id: string
  nom: string
  slug: string
  created_at: string
  nombre_utilisations: number
}

export interface CreerTagForm {
  nom: string
}

// ── Referentiels — Medias ────────────────────────────────────

export interface AdminMedia {
  id: string
  nom_original: string
  url_publique: string | null
  type_mime: string | null
  taille_octets: number | null
  created_at: string
}

export interface AdminMediaDetail {
  id: string
  nom_original: string
  chemin_stockage: string
  url_publique: string | null
  type_mime: string | null
  taille_octets: number | null
  largeur: number | null
  hauteur: number | null
  duree_secondes: number | null
  uploaded_by: string | null
  uploaded_by_nom: string | null
  created_at: string
}

// ── Referentiels — Specialites Biblio ────────────────────────

export interface AdminSpecialite {
  id: string
  nom: string
  slug: string
}

export interface AdminSpecialiteDetail {
  id: string
  nom: string
  slug: string
  nombre_utilisateurs: number
}

export interface CreerSpecialiteForm {
  nom: string
}

// ── Programmes d'echange ───────────────────────────────────

export interface AdminProgramme {
  id: string
  titre: string
  etat: string
  duree: string | null
  date_debut: string | null
  date_fin: string | null
  nombre_places: number | null
  pays_nom: string | null
  domaine_nom: string | null
  cree_par_nom: string | null
  created_at: string
}

export interface AdminProgrammeDetail {
  id: string
  titre: string
  slug: string | null
  description: string | null
  image_couverture_url: string | null
  document_legal_url: string | null
  pays_id: string | null
  pays_nom: string | null
  ville: string | null
  adresse: string | null
  prise_en_charge_billet: boolean
  prise_en_charge_hebergement: boolean
  prise_en_charge_subsistance: boolean
  prise_en_charge_details: string | null
  duree: string | null
  domaine_id: string | null
  domaine_nom: string | null
  date_debut: string | null
  date_fin: string | null
  nombre_places: number | null
  prerequis: string | null
  langues_requises: string[] | null
  etat: string
  cree_par: string | null
  cree_par_nom: string | null
  valide_par: string | null
  valide_par_nom: string | null
  valide_at: string | null
  nombre_candidatures: number
  created_at: string
  updated_at: string | null
}

export interface CreerProgrammeForm {
  titre: string
  description: string
  pays_id: string
  ville: string
  adresse: string
  prise_en_charge_billet: boolean
  prise_en_charge_hebergement: boolean
  prise_en_charge_subsistance: boolean
  prise_en_charge_details: string
  duree: string
  domaine_id: string
  date_debut: string
  date_fin: string
  nombre_places: number | null
  prerequis: string
  langues_requises: string[]
}

// ── Candidatures ───────────────────────────────────────────

export interface AdminCandidature {
  id: string
  statut: string
  programme_titre: string
  candidat_nom: string
  candidat_email: string
  created_at: string
}

export interface AdminCandidatureDetail {
  id: string
  programme_id: string
  programme_titre: string
  candidat_id: string
  candidat_nom: string
  candidat_prenom: string
  candidat_email: string
  candidat_photo_url: string | null
  lettre_motivation: string | null
  cv_url: string | null
  statut: string
  notes_internes: string | null
  traite_par: string | null
  traite_par_nom: string | null
  created_at: string
  updated_at: string | null
}

// ── Marche Africain — Annonces ────────────────────────────

export interface AdminAnnonce {
  id: string
  titre: string
  slug: string | null
  type_operation: string
  etat: string
  condition_article: string | null
  prix: number | null
  devise: string | null
  ville: string | null
  nombre_vues: number
  quantite: number | null
  cree_par: string
  created_at: string
  categorie_nom: string | null
  auteur_nom: string
  auteur_prenom: string
  pays_nom: string | null
  photo_url: string | null
}

export interface AdminAnnoncePays {
  pays_id: string
  pays_nom: string
}

export interface AdminAnnonceMedia {
  id: string
  media_url: string
  type_mime: string | null
  est_principale: boolean | null
  ordre: number | null
  created_at: string
}

export interface AdminAnnonceDetail {
  id: string
  titre: string
  slug: string | null
  description: string
  type_operation: string
  etat: string
  condition_article: string | null
  prix: number | null
  devise: string | null
  prix_negociable: boolean | null
  ville: string | null
  adresse: string | null
  longitude: number | null
  latitude: number | null
  type_contact: string | null
  contact_info: string | null
  quantite: number | null
  nombre_vues: number
  cree_par: string
  expire_at: string | null
  created_at: string
  updated_at: string
  categorie_id: string | null
  categorie_nom: string | null
  auteur_nom: string
  auteur_prenom: string
  auteur_email: string
  pays: AdminAnnoncePays[]
  medias: AdminAnnonceMedia[]
}

export interface CreerAnnonceForm {
  titre: string
  description: string
  type_operation: string
  categorie_id: string
  condition_article: string
  prix: number | null
  devise: string
  prix_negociable: boolean
  ville: string
  adresse: string
  longitude: number | null
  latitude: number | null
  type_contact: string
  contact_info: string
  quantite: number
  etat: string
  expire_at: string
}

// ── Marche Africain — Favoris ─────────────────────────────

export interface AdminFavori {
  utilisateur_id: string
  annonce_id: string
  created_at: string
  utilisateur_nom: string
  utilisateur_prenom: string
  utilisateur_email: string
  annonce_titre: string
  annonce_etat: string
}

export interface AdminFavoriStats {
  annonce_id: string
  annonce_titre: string
  nombre_favoris: number
}

// ── Innovation — Innovations ─────────────────────────────────

export interface AdminInnovation {
  id: string
  titre: string
  slug: string | null
  etat: string
  image_couverture_url: string | null
  ville: string | null
  nombre_vues: number
  cree_par: string
  created_at: string
  domaine_nom: string | null
  organisation_nom: string | null
  pays_nom: string | null
  auteur_nom: string
  auteur_prenom: string
}

export interface AdminInnovationMedia {
  id: string
  media_url: string
  type_mime: string | null
  ordre: number | null
  created_at: string
}

export interface AdminInnovationDetail {
  id: string
  titre: string
  slug: string | null
  description: string | null
  etat: string
  image_couverture_url: string | null
  ville: string | null
  nombre_vues: number
  domaine_id: string | null
  organisation_id: string | null
  pays_id: string | null
  cree_par: string
  created_at: string
  updated_at: string
  domaine_nom: string | null
  organisation_nom: string | null
  pays_nom: string | null
  auteur_nom: string
  auteur_prenom: string
  auteur_email: string
  medias: AdminInnovationMedia[]
}

export interface CreerInnovationForm {
  titre: string
  description: string
  image_couverture_url: string
  domaine_id: string
  organisation_id: string
  pays_id: string
  ville: string
  etat: string
}

// ── Innovation — Projets ────────────────────────────────────

export interface AdminProjet {
  id: string
  titre: string
  slug: string | null
  etat: string
  nom_organisation: string | null
  cout_total: number | null
  devise: string | null
  duree_mois: number | null
  cree_par: string
  created_at: string
  pays_nom: string | null
  auteur_nom: string
  auteur_prenom: string
}

export interface AdminProjetDocument {
  id: string
  nom: string
  url: string
  type_mime: string | null
  created_at: string
}

export interface AdminProjetDetail {
  id: string
  titre: string
  slug: string | null
  nom_organisation: string | null
  description_organisation: string | null
  site_web: string | null
  pays_id: string | null
  ville: string | null
  contact_email: string | null
  contact_telephone: string | null
  cout_total: number | null
  devise: string | null
  duree_mois: number | null
  date_commencement_souhaitee: string | null
  description: string
  objectifs: string
  resultats_attendus: string | null
  activites_programmees: string | null
  echeanciers: string | null
  contribution_autonomisation: string | null
  difficultes_risques: string | null
  etat: string
  cree_par: string
  traite_par: string | null
  created_at: string
  updated_at: string
  pays_nom: string | null
  auteur_nom: string
  auteur_prenom: string
  auteur_email: string
  traite_par_nom: string | null
  traite_par_prenom: string | null
  documents: AdminProjetDocument[]
}

export interface CreerProjetForm {
  titre: string
  description: string
  objectifs: string
  nom_organisation: string
  description_organisation: string
  site_web: string
  pays_id: string
  ville: string
  contact_email: string
  contact_telephone: string
  cout_total: number | null
  devise: string
  duree_mois: number | null
  date_commencement_souhaitee: string
  resultats_attendus: string
  activites_programmees: string
  echeanciers: string
  contribution_autonomisation: string
  difficultes_risques: string
}

// ── Innovation — Africantives ───────────────────────────────

export interface AdminAfricantive {
  id: string
  titre: string
  slug: string | null
  etat: string
  image_couverture_url: string | null
  ville: string | null
  cree_par: string
  created_at: string
  domaine_nom: string | null
  pays_nom: string | null
  auteur_nom: string
  auteur_prenom: string
}

export interface AdminAfricantiveDetail {
  id: string
  titre: string
  slug: string | null
  description: string
  etat: string
  image_couverture_url: string | null
  ville: string | null
  domaine_id: string | null
  pays_id: string | null
  cree_par: string
  created_at: string
  updated_at: string
  domaine_nom: string | null
  pays_nom: string | null
  auteur_nom: string
  auteur_prenom: string
  auteur_email: string
}

export interface CreerAfricantiveForm {
  titre: string
  description: string
  image_couverture_url: string
  domaine_id: string
  pays_id: string
  ville: string
  etat: string
}

// ── Culture — Centres culturels ─────────────────────────────

export interface AdminCentreCulturel {
  id: string
  nom: string
  ville: string | null
  pays_nom: string | null
  actif: boolean
  nombre_membres: number | null
  nombre_programmations: number | null
  created_at: string
}

export interface AdminMembreCentre {
  id: string
  utilisateur_id: string
  utilisateur_nom: string
  utilisateur_prenom: string
  role: string
  created_at: string
}

export interface AdminCentreCulturelDetail {
  id: string
  nom: string
  slug: string | null
  description: string | null
  image_couverture_url: string | null
  pays_id: string | null
  ville: string | null
  adresse: string | null
  longitude: number | null
  latitude: number | null
  actif: boolean
  cree_par: string
  cree_par_nom: string
  created_at: string
  updated_at: string
  pays_nom: string | null
  membres: AdminMembreCentre[]
}

export interface CreerCentreCulturelForm {
  nom: string
  description: string
  image_couverture_url: string
  pays_id: string
  ville: string
  adresse: string
  longitude: number | null
  latitude: number | null
}

// ── Culture — Programmations ────────────────────────────────

export interface AdminProgrammation {
  id: string
  titre: string
  lieu: string | null
  mode: string | null
  date_heure_debut: string
  date_heure_fin: string | null
  nombre_places: number | null
  centre_nom: string | null
  created_at: string
}

export interface AdminProgrammationDetail {
  id: string
  centre_culturel_id: string
  titre: string
  description: string | null
  lieu: string | null
  mode: string | null
  lien_en_ligne: string | null
  date_heure_debut: string
  date_heure_fin: string | null
  nombre_places: number | null
  cree_par: string
  cree_par_nom: string
  centre_nom: string | null
  created_at: string
  updated_at: string
}

export interface CreerProgrammationForm {
  centre_culturel_id: string
  titre: string
  description: string
  lieu: string
  mode: string
  lien_en_ligne: string
  date_heure_debut: string
  date_heure_fin: string
  nombre_places: number | null
}

// ── Culture — Codi-Moi ──────────────────────────────────────

export interface AdminCodimoi {
  id: string
  type_codimoi: string | null
  contenu: string
  etat: string
  nombre_likes: number
  nombre_dislikes: number
  pays_nom: string | null
  groupe_ethnique: string | null
  auteur_nom: string
  auteur_prenom: string
  created_at: string
}

export interface AdminCodimoiTag {
  tag_id: string
  tag_nom: string
}

export interface AdminCodimoiDetail {
  id: string
  type_codimoi: string | null
  contenu: string
  explication: string | null
  nom_auteur_originel: string | null
  pays_id: string | null
  groupe_ethnique: string | null
  couleur_fond: string | null
  image_couverture_url: string | null
  image_arriere_plan_url: string | null
  etat: string
  nombre_likes: number
  nombre_dislikes: number
  cree_par: string
  cree_par_nom: string
  created_at: string
  updated_at: string
  pays_nom: string | null
  tags: AdminCodimoiTag[]
  nombre_commentaires: number
}

export interface AdminCodimoiCommentaire {
  id: string
  parent_id: string | null
  contenu: string
  cree_par: string
  auteur_nom: string
  nombre_likes: number
  created_at: string
  supprime: boolean
  enfants: AdminCodimoiCommentaire[]
}

export interface CreerCodimoiForm {
  type_codimoi: string
  contenu: string
  explication: string
  nom_auteur_originel: string
  pays_id: string
  groupe_ethnique: string
  couleur_fond: string
  image_couverture_url: string
  image_arriere_plan_url: string
  etat: string
}
