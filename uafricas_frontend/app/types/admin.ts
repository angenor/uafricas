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

// ── Gouvernance — Factcheck ──────────────────────────────────

export interface AdminFactcheck {
  id: string
  contenu: string
  verdict: string | null
  etat: string
  nombre_likes: number
  nombre_dislikes: number
  pays_nom: string | null
  auteur_nom: string
  auteur_prenom: string
  created_at: string
}

export interface AdminFactcheckCommentaire {
  id: string
  parent_id: string | null
  contenu: string
  type_commentaire: string
  cree_par: string
  auteur_nom: string
  nombre_likes: number
  created_at: string
  supprime: boolean
  enfants: AdminFactcheckCommentaire[]
}

export interface AdminFactcheckDetail {
  id: string
  contenu: string
  source_originale: string | null
  verdict: string | null
  image_couverture_url: string | null
  couleur_fond: string | null
  etat: string
  nombre_likes: number
  nombre_dislikes: number
  pays_id: string | null
  cree_par: string
  cree_par_nom: string
  created_at: string
  updated_at: string
  pays_nom: string | null
  nombre_commentaires: number
}

export interface CreerFactcheckForm {
  contenu: string
  source_originale: string
  verdict: string
  image_couverture_url: string
  couleur_fond: string
  pays_id: string
  etat: string
}

export interface AdminFactcheckReactions {
  nombre_likes: number
  nombre_dislikes: number
}

// ── Gouvernance — Bad Habits (Mauvaises pratiques) ───────────

export interface AdminBadHabit {
  id: string
  titre: string
  slug: string | null
  categorie_probleme: string
  gravite: string
  etat: string
  nombre_soutiens: number
  publication_anonyme: boolean
  pays_nom: string | null
  auteur_nom: string
  auteur_prenom: string
  created_at: string
}

export interface AdminBadHabitMedia {
  id: string
  media_url: string
  type_mime: string | null
  ordre: number | null
  created_at: string
}

export interface AdminBadHabitDetail {
  id: string
  titre: string
  slug: string | null
  description_generale: string
  details_problematique: string
  categorie_probleme: string
  categorie_probleme_detail: string | null
  gravite: string
  preuves_temoignages: string | null
  solutions_proposees: string | null
  publication_anonyme: boolean
  geolocalisation_autorisee: boolean
  longitude: number | null
  latitude: number | null
  region: string | null
  ville_quartier_zone: string | null
  etat: string
  nombre_soutiens: number
  pays_id: string | null
  cree_par: string
  created_at: string
  updated_at: string
  pays_nom: string | null
  auteur_nom: string
  auteur_prenom: string
}

export interface CreerBadHabitForm {
  titre: string
  categorie_probleme: string
  categorie_probleme_detail: string
  gravite: string
  description_generale: string
  details_problematique: string
  preuves_temoignages: string
  solutions_proposees: string
  publication_anonyme: boolean
  geolocalisation_autorisee: boolean
  longitude: number | null
  latitude: number | null
  pays_id: string
  region: string
  ville_quartier_zone: string
  etat: string
}

// ── Gouvernance — Idea Forces ─────────────────────────────────

export interface AdminIdeaForce {
  id: string
  titre: string
  slug: string | null
  categorie_proposition: string
  urgence: string
  etat: string
  nombre_soutiens: number
  pays_nom: string | null
  auteur_nom: string
  auteur_prenom: string
  created_at: string
}

export interface AdminIdeaForceMedia {
  id: string
  media_url: string
  type_mime: string | null
  ordre: number | null
  created_at: string
}

export interface AdminIdeaForceDetail {
  id: string
  titre: string
  slug: string | null
  description_generale: string
  details_proposition: string
  categorie_proposition: string
  categorie_proposition_detail: string | null
  urgence: string
  plan_implementation: string | null
  ressources_necessaires: string | null
  impact_attendu: string | null
  etat: string
  nombre_soutiens: number
  region: string | null
  ville_quartier_zone: string | null
  pays_id: string | null
  cree_par: string
  created_at: string
  updated_at: string
  pays_nom: string | null
  auteur_nom: string
  auteur_prenom: string
}

export interface CreerIdeaForceForm {
  titre: string
  description_generale: string
  details_proposition: string
  categorie_proposition: string
  categorie_proposition_detail: string
  urgence: string
  plan_implementation: string
  ressources_necessaires: string
  impact_attendu: string
  pays_id: string
  region: string
  ville_quartier_zone: string
  etat: string
}

// ── Médias & Contenus — Radio & TV ───────────────────────────

export interface AdminStationRadio {
  id: string
  nom: string
  type_station: string
  genre: string | null
  etat: string
  pays_nom: string | null
  ville: string | null
  created_at: string
}

export interface AdminStationRadioDetail {
  id: string
  nom: string
  slug: string | null
  description: string | null
  stream_url: string
  image_couverture_url: string | null
  genre: string | null
  genres_liste: string[]
  pays_id: string | null
  pays_nom: string | null
  ville: string | null
  type_station: string
  etat: string
  cree_par: string
  cree_par_nom: string | null
  created_at: string
  updated_at: string
}

export interface CreerStationRadioForm {
  nom: string
  description: string
  stream_url: string
  image_couverture_url: string
  genre: string
  genres_liste: string[]
  pays_id: string
  ville: string
  type_station: string
}

export interface AdminChaineTv {
  id: string
  nom: string
  categorie: string
  etat: string
  est_en_direct: boolean
  pays_nom: string | null
  langue: string
  created_at: string
}

export interface AdminChaineTvDetail {
  id: string
  nom: string
  slug: string | null
  description: string | null
  stream_url: string
  image_couverture_url: string | null
  categorie: string
  pays_id: string | null
  pays_nom: string | null
  langue: string
  est_en_direct: boolean
  etat: string
  cree_par: string
  cree_par_nom: string | null
  created_at: string
  updated_at: string
}

export interface CreerChaineTvForm {
  nom: string
  description: string
  stream_url: string
  image_couverture_url: string
  categorie: string
  pays_id: string
  langue: string
  est_en_direct: boolean
}

export interface AdminProgrammeMedia {
  id: string
  nom_emission: string
  type_programme: string
  etat: string
  categorie_radio: string | null
  langue: string
  pays_nom: string | null
  created_at: string
}

export interface AdminProgrammeMediaDetail {
  id: string
  nom_emission: string
  slug: string | null
  type_programme: string
  description: string
  image_couverture_url: string | null
  video_url: string | null
  info_animateur: string | null
  info_producteur: string | null
  pays_id: string | null
  pays_nom: string | null
  est_international: boolean
  langue: string
  categorie_radio: string | null
  etat: string
  cree_par: string
  cree_par_nom: string | null
  created_at: string
  updated_at: string
}

export interface CreerProgrammeMediaForm {
  nom_emission: string
  type_programme: string
  description: string
  image_couverture_url: string
  video_url: string
  info_animateur: string
  info_producteur: string
  pays_id: string
  est_international: boolean
  langue: string
  categorie_radio: string
}

// ── Médias & Contenus — Événements ───────────────────────────

export interface AdminEvenement {
  id: string
  titre: string
  format: string
  etat: string
  date_heure_debut: string
  date_heure_fin: string | null
  nombre_places: number | null
  pays_nom: string | null
  ville: string | null
  cree_par_nom: string | null
  created_at: string
}

export interface AdminEvenementDetail {
  id: string
  titre: string
  slug: string | null
  description: string
  type_evenement: string | null
  pays_id: string | null
  pays_nom: string | null
  ville: string | null
  adresse: string | null
  date_heure_debut: string
  date_heure_fin: string | null
  image_couverture_url: string | null
  format: string
  lien_en_ligne: string | null
  langue: string
  nombre_places: number | null
  etat: string
  cree_par: string
  cree_par_nom: string | null
  nombre_inscriptions: number
  created_at: string
  updated_at: string
}

export interface CreerEvenementAdminForm {
  titre: string
  description: string
  type_evenement: string
  pays_id: string
  ville: string
  adresse: string
  date_heure_debut: string
  date_heure_fin: string
  image_couverture_url: string
  format: string
  lien_en_ligne: string
  langue: string
  nombre_places: number | null
}

export interface AdminEvenementInscription {
  id: string
  utilisateur_id: string
  nom: string
  prenom: string
  email: string
  statut: string
  created_at: string
}

export interface AdminEvenementInscriptionStats {
  total: number
  inscrits: number
  confirmes: number
  annules: number
  presents: number
  absents: number
}

// ── Médias & Contenus — MOOC ─────────────────────────────────

export interface AdminMooc {
  id: string
  titre: string
  format: string
  etat: string
  date_heure_debut: string
  date_heure_fin: string | null
  nombre_places: number | null
  pays_nom: string | null
  cree_par_nom: string | null
  created_at: string
}

export interface AdminMoocDetail {
  id: string
  titre: string
  slug: string | null
  description: string
  type_formation: string | null
  pays_id: string | null
  pays_nom: string | null
  ville: string | null
  date_heure_debut: string
  date_heure_fin: string | null
  image_couverture_url: string | null
  format: string
  lien_en_ligne: string | null
  langue: string
  nombre_places: number | null
  prerequis: string | null
  etat: string
  cree_par: string
  cree_par_nom: string | null
  nombre_inscriptions: number
  progression_moyenne: number
  created_at: string
  updated_at: string
}

export interface CreerMoocForm {
  titre: string
  description: string
  type_formation: string
  pays_id: string
  ville: string
  date_heure_debut: string
  date_heure_fin: string
  image_couverture_url: string
  format: string
  lien_en_ligne: string
  langue: string
  nombre_places: number | null
  prerequis: string
}

export interface AdminMoocInscription {
  id: string
  utilisateur_id: string
  nom: string
  prenom: string
  email: string
  statut: string
  progression: number
  created_at: string
}

export interface AdminMoocInscriptionStats {
  total: number
  inscrits: number
  en_cours: number
  completes: number
  abandonnes: number
  progression_moyenne: number
}

// ── Médias & Contenus — Bibliothèque (Livres) ───────────────

export interface AdminLivre {
  id: string
  titre: string
  type_document: string
  acces: string
  etat: string
  info_auteur: string
  nombre_vues: number
  nombre_telechargements: number
  categorie_nom: string | null
  cree_par_nom: string | null
  created_at: string
}

export interface AdminLivreDetail {
  id: string
  titre: string
  slug: string | null
  description: string
  image_couverture_url: string | null
  document_pdf_url: string
  type_document: string
  categorie_id: string | null
  categorie_nom: string | null
  acces: string
  info_auteur: string
  date_publication: string | null
  rapport_auteur: string | null
  condition_diffusion: string | null
  acceptation_diffusion: boolean
  langue: string | null
  nombre_pages: number | null
  isbn: string | null
  nombre_telechargements: number
  nombre_vues: number
  etat: string
  cree_par: string
  cree_par_nom: string | null
  tags: AdminLivreTag[]
  created_at: string
  updated_at: string
}

export interface AdminLivreTag {
  tag_id: string
  tag_nom: string
}

export interface CreerLivreAdminForm {
  titre: string
  description: string
  image_couverture_url: string
  document_pdf_url: string
  type_document: string
  categorie_id: string
  acces: string
  info_auteur: string
  date_publication: string
  rapport_auteur: string
  condition_diffusion: string
  acceptation_diffusion: boolean
  langue: string
  nombre_pages: number | null
  isbn: string
}

// ── Profils Pays (Country Profile) ──────────────────────────

export interface AdminFichePay {
  id: string
  pays_id: string
  pays_nom: string
  pays_code: string | null
  image_drapeau_url: string | null
  slogan: string | null
  population: number | null
  superficie_km2: number | null
  cree_par_nom: string | null
  created_at: string
}

export interface AdminFichePayDetail {
  id: string
  pays_id: string
  pays_nom: string
  pays_code: string | null
  image_couverture_url: string | null
  slogan: string | null
  superficie_km2: number | null
  population: number | null
  biographie: string | null
  contexte: string | null
  contexte_historique: string | null
  image_drapeau_url: string | null
  image_embleme_url: string | null
  image_devise_url: string | null
  hymne_national: string | null
  langue_officielle: string | null
  langues_populaires: string | null
  monnaie: string | null
  fuseau_horaire: string | null
  bloquee: boolean
  nombre_signalements: number
  cree_par: string
  cree_par_nom: string | null
  nb_regions: number
  nb_groupes_ethniques: number
  nb_alliances: number
  nb_contes: number
  nb_sites_touristiques: number
  nb_secteurs: number
  nb_saisons: number
  nb_liens_interethniques: number
  created_at: string
  updated_at: string
}

export interface CreerFichePayForm {
  pays_id: string
  slogan: string
  superficie_km2: number | null
  population: number | null
  biographie: string
  contexte: string
  contexte_historique: string
  image_couverture_url: string
  image_drapeau_url: string
  image_embleme_url: string
  image_devise_url: string
  hymne_national: string
  langue_officielle: string
  langues_populaires: string
  monnaie: string
  fuseau_horaire: string
}

// ── Profils Pays — Sous-entites ─────────────────────────────

export interface AdminRegion {
  id: string
  fiche_pays_id: string
  nom: string
  chef_lieu: string | null
  description: string | null
  population: number | null
  created_at: string
  updated_at: string
}

export interface AdminGroupeEthnique {
  id: string
  fiche_pays_id: string
  nom: string
  description: string | null
  objets_culturels_distinctifs: string | null
  population_estimee: string | null
  langues: string | null
  region_id: string | null
  region_nom: string | null
  created_at: string
  updated_at: string
}

export interface AdminAlliance {
  id: string
  fiche_pays_id: string
  nom: string
  description: string | null
  groupes_impliques: string | null
  signification: string | null
  created_at: string
  updated_at: string
}

export interface AdminConte {
  id: string
  fiche_pays_id: string
  titre: string
  contenu: string | null
  type_conte: string | null
  groupe_ethnique_id: string | null
  groupe_ethnique_nom: string | null
  image_url: string | null
  created_at: string
  updated_at: string
}

export interface AdminSiteTouristique {
  id: string
  fiche_pays_id: string
  nom: string
  categorie: string | null
  sous_type: string | null
  description: string | null
  info_pertinente: string | null
  image_url: string | null
  gestionnaire: string | null
  ville: string | null
  village: string | null
  longitude: number | null
  latitude: number | null
  contact_telephone: string | null
  contact_courriel: string | null
  contact_adresse: string | null
  constitution_statut_juridique: string | null
  constitution_numero: string | null
  constitution_document_url: string | null
  verifie: boolean
  region_id: string | null
  region_nom: string | null
  created_at: string
  updated_at: string
}

export interface AdminSecteur {
  id: string
  fiche_pays_id: string
  nom: string
  description: string | null
  created_at: string
}

export interface AdminSaison {
  id: string
  fiche_pays_id: string
  nom: string
  description: string | null
  mois_debut: number | null
  mois_fin: number | null
  created_at: string
}

export interface AdminLienInterethnique {
  id: string
  fiche_pays_id: string
  pays_lie_id: string | null
  pays_lie_nom: string | null
  description: string | null
  type_lien: string | null
  created_at: string
  updated_at: string
}

// ── Profils Pays — Contributions ────────────────────────────

export interface AdminContribution {
  id: string
  fiche_pays_id: string
  pays_nom: string
  section: string
  type_contribution: string
  etat: string
  contributeur_nom: string | null
  traite_par_nom: string | null
  created_at: string
  traite_at: string | null
}

export interface AdminContributionPieceJointe {
  chemin_fichier: string
  legende: string
  format: string
  taille_octets: number
  largeur: number
  hauteur: number
  url_signee: string
}

export interface AdminContributionConcurrente {
  id: string
  cree_par_nom: string | null
  created_at: string
}

export interface AdminContributionDetail {
  id: string
  fiche_pays_id: string
  pays_nom: string
  section: string
  type_contribution: string
  ancienne_valeur: string | null
  nouvelle_valeur: string | null
  justification: string | null
  etat: string
  cree_par: string
  contributeur_nom: string | null
  traite_par: string | null
  traite_par_nom: string | null
  note_moderation: string | null
  traite_at: string | null
  created_at: string
  updated_at: string
  // Afripulse (T040)
  type_objet_contribution?: string
  section_afripulse?: string | null
  target_id?: string | null
  nouvelle_valeur_jsonb?: unknown | null
  ancienne_valeur_jsonb?: unknown | null
  pieces_jointes?: AdminContributionPieceJointe[]
  contributions_concurrentes?: AdminContributionConcurrente[]
}

// ── Audit & Logs ────────────────────────────────────────────

export interface AdminAudit {
  id: string
  action: string
  schema_name: string
  table_name: string
  record_id: string | null
  ip_address: string | null
  created_at: string
  utilisateur_nom: string | null
  utilisateur_id: string | null
}

export interface AdminAuditDetail {
  id: string
  action: string
  schema_name: string
  table_name: string
  record_id: string | null
  ancien_etat: Record<string, any> | null
  nouvel_etat: Record<string, any> | null
  ip_address: string | null
  user_agent: string | null
  created_at: string
  utilisateur_id: string | null
  utilisateur_nom: string | null
  utilisateur_email: string | null
}

// ── Dashboard ────────────────────────────────────────────────

export interface DashboardStatsSimple {
  total: number
}

export interface DashboardStatsUtilisateurs {
  total: number
  actifs: number
  en_attente: number
  suspendus: number
}

export interface DashboardStatsAnnonces {
  total: number
  publiees: number
  en_attente: number
  expirees: number
}

export interface DashboardStatsProgrammes {
  total: number
  actifs: number
  candidatures_en_attente: number
}

export interface DashboardStatsInnovations {
  total: number
  publiees: number
}

export interface DashboardStatsProjets {
  total: number
  approuves: number
  en_revue: number
  soumis: number
}

export interface DashboardStatsCodimoi {
  total: number
  par_type: {
    proverbe_adage: number
    citation: number
    ressource_historique: number
    bonne_pratique: number
  }
}

export interface DashboardStatsSessionsAfrolang {
  total: number
  en_cours: number
}

export interface DashboardStatsEvenements {
  total: number
  a_venir: number
  inscrits_total: number
}

export interface DashboardStatsMoocs {
  total: number
  inscrits_total: number
  en_cours: number
}

export interface DashboardStatsRadioTv {
  stations_radio: number
  chaines_tv: number
}

export interface DashboardStatsFactchecks {
  total: number
  par_verdict: {
    vrai: number
    faux: number
    partiellement_vrai: number
    trompeur: number
    non_verifie: number
  }
}

export interface DashboardStatsBadHabits {
  total: number
  par_gravite: {
    faible: number
    elevee: number
    critique: number
  }
}

export interface DashboardStatsFichesPays {
  total: number
  contributions_en_attente: number
}

export interface DashboardStatsAudit {
  actions_aujourd_hui: number
  actions_cette_semaine: number
}

export interface DashboardStats {
  utilisateurs: DashboardStatsUtilisateurs
  organisations: DashboardStatsSimple
  annonces: DashboardStatsAnnonces
  programmes: DashboardStatsProgrammes
  innovations: DashboardStatsInnovations
  projets: DashboardStatsProjets
  africantives: DashboardStatsSimple
  centres_culturels: DashboardStatsSimple
  codimoi: DashboardStatsCodimoi
  sessions_afrolang: DashboardStatsSessionsAfrolang
  evenements: DashboardStatsEvenements
  moocs: DashboardStatsMoocs
  livres: DashboardStatsSimple
  radio_tv: DashboardStatsRadioTv
  factchecks: DashboardStatsFactchecks
  bad_habits: DashboardStatsBadHabits
  idea_forces: DashboardStatsSimple
  fiches_pays: DashboardStatsFichesPays
  audit: DashboardStatsAudit
}

export interface DashboardActiviteItem {
  id: string
  action: string
  schema_name: string
  table_name: string
  record_id: string | null
  created_at: string
  utilisateur_nom: string | null
  utilisateur_id: string | null
}

export interface DashboardTendancePoint {
  jour: string
  total: number
}

export interface DashboardTendances {
  periode: string
  inscriptions_utilisateurs: DashboardTendancePoint[]
  annonces_publiees: DashboardTendancePoint[]
  evenements: DashboardTendancePoint[]
  contributions_pays: DashboardTendancePoint[]
}
