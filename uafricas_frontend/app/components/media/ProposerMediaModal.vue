<script setup lang="ts">
/**
 * Soumission d'un média par une partie prenante (US4).
 *
 * Remplace `AddProgramModal.vue`, maquette morte dont le `handleSubmit` se
 * contentait de simuler l'envoi.
 *
 * **Rien de ce qui est saisi ici n'atteint le public directement** : la
 * proposition part en `'en_attente'` et n'existe dans les tables métier
 * qu'après validation par un administrateur (FR-031). Le formulaire le dit
 * explicitement, pour ne pas laisser croire à une publication immédiate.
 */
import {
  useMediaProposition,
  ROLES_PARTIE_PRENANTE,
  LIBELLES_TYPE_OBJET,
  type TypeObjetPropose,
  type DonneesProposition,
  type ThemePhareAPI,
} from '~/composables/useMediaProposition'
import type { ThematiquePublique, TerritoirePublic } from '~/composables/useMediaSupport'

const props = withDefaults(defineProps<{
  isOpen: boolean
  /** Types offerts par la page appelante (la page Télé n'offre pas la radio). */
  typesOfferts?: TypeObjetPropose[]
  /** Support visé, quand la proposition part d'une page de détail. */
  targetId?: string | null
}>(), {
  typesOfferts: () => ['chaine_tv', 'station_radio', 'emission_tele', 'emission_radio'],
  targetId: null,
})

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'soumise'): void
}>()

const { soumettre, chargement, erreur: erreurApi, listerThemes } = useMediaProposition()
const { listerReferentielsEdition } = useMediaSupport()
const { listerChaines } = useTelevision()
const { listerStations } = useStationsRadio()
const userStore = useUserStore()

const typeObjet = ref<TypeObjetPropose>(props.typesOfferts[0] ?? 'chaine_tv')
const justification = ref('')
const donnees = ref<DonneesProposition>({})
const fichierMedia = ref<File | null>(null)
const fichierImage = ref<File | null>(null)
const lienExterne = ref('')
const erreur = ref('')
const succes = ref(false)

// Référentiels chargés une fois à la première ouverture.
const themes = ref<ThemePhareAPI[]>([])
/**
 * Territoires proposables. Ils viennent du RÉFÉRENTIEL D'ÉDITION, borné aux 55
 * territoires africains, et non de `/api/pays`, qui en compte 198 : proposer
 * l'Albanie comme couverture d'une chaîne panafricaine noyait les seuls choix
 * qui aient un sens ici.
 */
const territoires = ref<TerritoirePublic[]>([])
/**
 * Thématiques de GRILLE, pour une chaîne thématique. Les lignes éditoriales
 * d'Africans Télé International (09u) arrivent dans la même liste et sont
 * écartées : elles relèvent d'un choix éditorial de la plateforme, jamais de
 * la nature d'une chaîne proposée par un tiers.
 */
const thematiquesSupport = ref<ThematiquePublique[]>([])
/**
 * Valeur du sélecteur de thème : l'identifiant d'un thème du référentiel, ou
 * la sentinelle `'autre'` qui révèle le champ de précision libre. Distincte de
 * `donnees.theme_phare_id`, qu'elle pilote, car « Autre » n'est pas un id.
 */
const themeSelection = ref('')
const AUTRE = 'autre'

/**
 * Chaînes ou stations publiées, pour rattacher une émission proposée. Une
 * émission sans support parent (`chaine_id`/`station_id` nul) n'apparaîtrait
 * sur aucune page : les pages Télé et Radio sont entièrement structurées par
 * support. Le rattachement est donc obligatoire, sauf si la proposition part
 * déjà d'une page de détail, qui fixe le parent par `targetId`.
 */
const supportsParents = ref<{ id: string, nom: string }[]>([])

const estSupport = computed(() => ['chaine_tv', 'station_radio'].includes(typeObjet.value))

/**
 * Nature du support proposé (09v). `false` = territoriale, le cas courant.
 *
 * Le drapeau vit ici plutôt que dans `donnees` parce qu'il pilote l'affichage
 * autant que l'envoi : basculer d'une nature à l'autre efface les champs de
 * l'autre, qui n'ont plus de sens.
 */
const estThematique = computed({
  get: () => donnees.value.est_thematique === true,
  set: (valeur: boolean) => {
    donnees.value.est_thematique = valeur
    if (valeur) {
      // Une chaîne thématique couvre tous les territoires : ce n'est pas une
      // case cochée par défaut, c'est une conséquence, et le serveur la
      // réécrira de toute façon.
      donnees.value.territoires = []
      donnees.value.couverture_continentale = undefined
    }
    else {
      donnees.value.thematique_id = undefined
    }
  },
})
// Depuis 009, « contenu » recouvre les PROGRAMMES conteneurs (`emission_*`) et
// les épisodes (`episode_*`) : les deux se rattachent à un support et portent un
// média, à ceci près qu'un programme peut naître sans fichier (FR-003).
const estContenu = computed(
  () => typeObjet.value.startsWith('emission_') || typeObjet.value.startsWith('episode_'),
)
const estVideo = computed(() => typeObjet.value.endsWith('_tele'))

/** Le parent est demandé pour une émission proposée hors d'une page de détail. */
const rattachementRequis = computed(() => estContenu.value && !props.targetId)

const chargerSupportsParents = async () => {
  supportsParents.value = []
  if (!rattachementRequis.value) return
  if (estVideo.value) {
    const res = await listerChaines({ par_page: 100 })
    supportsParents.value = (res?.chaines ?? []).map(c => ({ id: c.id, nom: c.name }))
  }
  else {
    const res = await listerStations({ par_page: 100 })
    supportsParents.value = (res?.stations ?? []).map(s => ({ id: s.id, nom: s.name }))
  }
}

/**
 * Identifiant du support parent choisi, dirigé vers `chaine_id` ou `station_id`
 * selon le type d'émission. La validation admin lit l'un ou l'autre.
 */
const parentSelection = ref('')
watch(parentSelection, (id) => {
  donnees.value.chaine_id = estVideo.value ? (id || undefined) : undefined
  donnees.value.station_id = estVideo.value ? undefined : (id || undefined)
})

// Le choix pilote les deux champs envoyés au serveur : un id, OU une précision
// libre : jamais les deux, à l'image du CHECK « Autre exige une précision ».
watch(themeSelection, (valeur) => {
  if (valeur === AUTRE) {
    donnees.value.theme_phare_id = undefined
  }
  else {
    donnees.value.theme_phare_id = valeur || undefined
    donnees.value.theme_phare_autre = undefined
  }
})

const reinitialiser = () => {
  etapeCourante.value = 0
  typeObjet.value = props.typesOfferts[0] ?? 'chaine_tv'
  justification.value = ''
  donnees.value = {}
  fichierMedia.value = null
  fichierImage.value = null
  lienExterne.value = ''
  themeSelection.value = ''
  parentSelection.value = ''
  erreur.value = ''
  succes.value = false
}

watch(() => props.isOpen, async (ouvert) => {
  if (!ouvert) return
  reinitialiser()
  if (themes.value.length === 0) themes.value = await listerThemes()
  if (thematiquesSupport.value.length === 0 || territoires.value.length === 0) {
    const referentiels = await listerReferentielsEdition()
    // Les lignes éditoriales d'Africans Télé International n'ont rien à faire
    // ici : elles relèvent d'un choix de la plateforme, pas d'un proposant.
    thematiquesSupport.value = referentiels.thematiques.filter(t => !t.est_ligne_editoriale)
    territoires.value = referentiels.territoires
  }
  await chargerSupportsParents()
})

// Changer de type invalide les champs propres à l'ancien.
watch(typeObjet, async () => {
  donnees.value = {}
  fichierMedia.value = null
  lienExterne.value = ''
  themeSelection.value = ''
  parentSelection.value = ''
  await chargerSupportsParents()
})

const surFichierMedia = (e: Event) => {
  fichierMedia.value = (e.target as HTMLInputElement).files?.[0] ?? null
  if (fichierMedia.value) lienExterne.value = ''
}
const surFichierImage = (e: Event) => {
  fichierImage.value = (e.target as HTMLInputElement).files?.[0] ?? null
}

const fermer = () => {
  if (chargement.value) return
  emit('close')
}

/**
 * Validation locale, doublée côté serveur ET par des CHECK SQL : ce qui suit
 * ne sert qu'à donner un message immédiat, jamais de garantie.
 */
const valider = (): string | null => {
  if (!donnees.value.nom?.trim()) return 'Le nom est requis.'
  if (!justification.value.trim()) {
    return 'Expliquez en quelques mots pourquoi vous proposez ce contenu.'
  }
  if (estSupport.value) {
    // Portée (09v). Le territoire n'est plus demandé pour un contenu : il
    // n'était écrit nulle part, une émission tenant son territoire de son
    // support.
    if (estThematique.value) {
      if (!donnees.value.thematique_id) return 'Choisissez la thématique de la chaîne.'
    }
    else if (
      !donnees.value.couverture_continentale
      && !(donnees.value.territoires?.length)
    ) {
      return 'Choisissez un ou plusieurs territoires, ou « Tous les territoires ».'
    }
    if (!donnees.value.role_partie_prenante) {
      return 'Indiquez à quel titre vous proposez ce média.'
    }
    if (
      donnees.value.role_partie_prenante === 'autre'
      && !donnees.value.role_partie_prenante_autre?.trim()
    ) {
      return 'Précisez le rôle choisi au titre de « Autre ».'
    }
  }
  if (rattachementRequis.value && !parentSelection.value) {
    return estVideo.value
      ? 'Choisissez la chaîne à laquelle rattacher cette émission.'
      : 'Choisissez la station à laquelle rattacher cette émission.'
  }
  if (estContenu.value) {
    if (!themeSelection.value) {
      return 'Choisissez un thème phare.'
    }
    if (themeSelection.value === AUTRE && !donnees.value.theme_phare_autre?.trim()) {
      return 'Précisez le thème phare choisi au titre de « Autre ».'
    }
  }
  return null
}

const ETAPES = [
  { titre: 'Le contenu' },
  { titre: 'Fichiers & source' },
  { titre: 'Contacts & envoi' },
] as const
const etapeCourante = ref(0)

/**
 * À quelle étape se corrige un message donné. `valider()` reste l'autorité
 * unique : ceci ne fait que router son message vers l'étape qui l'affiche —
 * un message rendu sur une étape invisible est un message perdu.
 */
const etapeDeLErreur = (message: string): number =>
  message.startsWith('Expliquez') ? 2 : 0

const suivant = () => {
  const probleme = valider()
  // Seule l'étape 1 bloque : c'est elle qui porte l'identité du contenu.
  if (etapeCourante.value === 0 && probleme && etapeDeLErreur(probleme) === 0) {
    erreur.value = probleme
    return
  }
  erreur.value = ''
  etapeCourante.value = Math.min(etapeCourante.value + 1, ETAPES.length - 1)
}

const soumettreFormulaire = async () => {
  const probleme = valider()
  if (probleme) {
    erreur.value = probleme
    etapeCourante.value = etapeDeLErreur(probleme)
    return
  }
  erreur.value = ''

  // Un lien externe et un fichier ne coexistent pas pour le même média.
  const payload: DonneesProposition = { ...donnees.value }
  if (lienExterne.value.trim() && !fichierMedia.value) {
    if (estVideo.value) payload.video_url = lienExterne.value.trim()
    else if (estContenu.value) payload.audio_url = lienExterne.value.trim()
    else payload.stream_url = lienExterne.value.trim()
  }

  const res = await soumettre({
    type_objet: typeObjet.value,
    target_id: props.targetId,
    justification: justification.value.trim(),
    donnees: payload,
    media: fichierMedia.value,
    image: fichierImage.value,
  })

  if (res) {
    succes.value = true
    emit('soumise')
    setTimeout(() => emit('close'), 2200)
  }
  else {
    erreur.value = erreurApi.value || 'Erreur lors de l’envoi. Veuillez réessayer.'
  }
}

</script>

<template>
  <AfricansModale
    :model-value="isOpen"
    titre="Proposer un contenu"
    sous-titre="Examiné par un administrateur avant publication"
    icone="fa-solid fa-paper-plane"
    taille="large"
    @update:model-value="fermer()"
  >
    <!-- Confirmation -->
    <div v-if="succes" class="flex flex-col items-center gap-3 py-6 text-center">
      <span class="grid size-14 place-items-center rounded-full bg-af-vert/10">
        <font-awesome-icon icon="fa-solid fa-check" class="text-2xl text-af-vert" />
      </span>
      <p class="text-base font-bold text-af-encre">Proposition envoyée !</p>
      <p class="max-w-sm text-[14px]/[1.6] text-af-corps">
        Elle sera examinée par un administrateur avant publication. Vous pouvez en
        suivre l'avancement depuis
        <NuxtLink to="/mon-compte/propositions-medias" class="font-bold text-af-chocolat underline">
          vos propositions
        </NuxtLink>.
      </p>
    </div>

    <!-- Invitation à se connecter -->
    <div v-else-if="!userStore.accessToken" class="flex flex-col items-center gap-3 py-8 text-center">
      <font-awesome-icon icon="fa-solid fa-lock" class="text-3xl text-af-atone-2" />
      <p class="text-[14px]/[1.4] text-af-corps">Proposer un contenu demande un compte.</p>
      <AfricansBouton vers="/login" variante="secondaire">Se connecter</AfricansBouton>
    </div>

    <template v-else>
      <AfricansEtapes :etapes="ETAPES" :courante="etapeCourante" class="mb-6" @aller="etapeCourante = $event" />

      <form id="form-proposition-media" class="flex flex-col gap-5" @submit.prevent="soumettreFormulaire">
        <!-- Rien de non validé n'est public : le dire d'emblée. -->
        <p class="flex gap-3 rounded-lg border border-af-chocolat/20 bg-af-chocolat/5 px-4 py-3 text-[14px]/[1.6] text-af-corps">
          <font-awesome-icon icon="fa-solid fa-circle-info" class="mt-1 shrink-0 text-af-chocolat" />
          <span>
            Votre proposition est examinée par un administrateur avant toute publication.
            Elle n'apparaîtra sur le site qu'une fois validée.
          </span>
        </p>

        <!-- ─── Étape 1 : identité du contenu ─── -->
        <template v-if="etapeCourante === 0">
          <AfricansChamp v-model="typeObjet" libelle="Que proposez-vous ?" type="select" obligatoire>
            <option v-for="t in typesOfferts" :key="t" :value="t">{{ LIBELLES_TYPE_OBJET[t] }}</option>
          </AfricansChamp>

          <AfricansChamp
            v-model="donnees.nom"
            libelle="Nom"
            :maxlength="350"
            placeholder="Nom de la chaîne, de la station ou de l'émission"
            obligatoire
          />

          <AfricansChamp
            v-model="donnees.description"
            libelle="Description"
            type="textarea"
            :lignes="3"
            placeholder="Présentez le contenu en quelques phrases…"
          />

          <!-- Portée du support (09v). Deux natures exclusives, et le
               formulaire ne montre jamais les champs de l'autre : une chaîne
               thématique n'a pas de territoire à saisir, elle les couvre tous.

               Le champ « Territoire » unique qui tenait cette place a disparu :
               pour un support il ne disait qu'un territoire là où la chaîne
               peut en couvrir plusieurs, et pour un contenu il n'était écrit
               nulle part — une émission tient son territoire de son support. -->
          <div v-if="estSupport" class="flex flex-col gap-3">
            <p class="text-[14px]/[1.4] text-af-corps">
              Portée de la {{ typeObjet === 'chaine_tv' ? 'chaîne' : 'station' }}
              <span class="text-af-live">*</span>
            </p>

            <div class="grid gap-2 sm:grid-cols-2">
              <button
                type="button"
                class="rounded-lg border px-4 py-3 text-left transition-colors"
                :class="!estThematique
                  ? 'border-af-chocolat bg-af-chocolat/10'
                  : 'border-af-bordure bg-af-surface hover:border-af-chocolat/50'"
                @click="estThematique = false"
              >
                <span class="block text-[14px]/[1.4] font-bold text-af-encre">Territoriale</span>
                <span class="mt-0.5 block text-[12px]/[1.4] text-af-atone">
                  Un ou plusieurs territoires, ou toute l'Afrique.
                </span>
              </button>

              <button
                type="button"
                class="rounded-lg border px-4 py-3 text-left transition-colors"
                :class="estThematique
                  ? 'border-af-chocolat bg-af-chocolat/10'
                  : 'border-af-bordure bg-af-surface hover:border-af-chocolat/50'"
                @click="estThematique = true"
              >
                <span class="block text-[14px]/[1.4] font-bold text-af-encre">Thématique</span>
                <span class="mt-0.5 block text-[12px]/[1.4] text-af-atone">
                  Une seule thématique, sur tous les territoires.
                </span>
              </button>
            </div>

            <MediaSelecteurCouverture
              v-if="!estThematique"
              :continentale="donnees.couverture_continentale === true"
              :territoires="donnees.territoires ?? []"
              :options="territoires"
              requis
              @update:continentale="donnees.couverture_continentale = $event"
              @update:territoires="donnees.territoires = $event"
            />

            <template v-else>
              <AfricansChamp
                v-model="donnees.thematique_id"
                libelle="Thématique de la chaîne"
                type="select"
                obligatoire
              >
                <option value="">Choisir une thématique…</option>
                <option v-for="t in thematiquesSupport" :key="t.id" :value="t.id">{{ t.nom }}</option>
              </AfricansChamp>
              <p class="text-[12px]/[1.4] text-af-atone">
                Aucun territoire à indiquer : une chaîne thématique concerne d'office
                tous les territoires.
              </p>
            </template>
          </div>

          <!-- Rôle de partie prenante (supports) : FR-029 -->
          <div v-if="estSupport" class="flex flex-col gap-2">
            <AfricansChamp
              v-model="donnees.role_partie_prenante"
              libelle="À quel titre proposez-vous ce média ?"
              type="select"
              obligatoire
            >
              <option value="">Choisir…</option>
              <option v-for="r in ROLES_PARTIE_PRENANTE" :key="r.valeur" :value="r.valeur">
                {{ r.libelle }}
              </option>
            </AfricansChamp>
            <AfricansChamp
              v-if="donnees.role_partie_prenante === 'autre'"
              v-model="donnees.role_partie_prenante_autre"
              libelle="Précisez votre rôle"
              :maxlength="200"
              obligatoire
            />
          </div>

          <!-- Support de rattachement (émission proposée hors page de détail).
               Sans lui, l'émission serait orpheline et n'apparaîtrait sur
               aucune page, toutes structurées par support. -->
          <div v-if="rattachementRequis">
            <AfricansChamp
              v-model="parentSelection"
              :libelle="estVideo ? 'Chaîne de rattachement' : 'Station de rattachement'"
              type="select"
              obligatoire
            >
              <option value="">{{ estVideo ? 'Choisir une chaîne…' : 'Choisir une station…' }}</option>
              <option v-for="parent in supportsParents" :key="parent.id" :value="parent.id">
                {{ parent.nom }}
              </option>
            </AfricansChamp>
            <p v-if="supportsParents.length === 0" class="mt-1.5 text-[12px] text-af-atone">
              Aucune {{ estVideo ? 'chaîne' : 'station' }} publiée pour l'instant.
              Proposez-en une d'abord, ou attendez sa validation.
            </p>
          </div>

          <!-- Thème phare (contenus) : FR-030 -->
          <div v-if="estContenu" class="flex flex-col gap-2">
            <AfricansChamp v-model="themeSelection" libelle="Thème phare" type="select" obligatoire>
              <option value="">Choisir un thème…</option>
              <option v-for="theme in themes" :key="theme.id" :value="theme.id">{{ theme.nom }}</option>
              <option :value="AUTRE">Autre (à préciser)</option>
            </AfricansChamp>
            <AfricansChamp
              v-if="themeSelection === AUTRE"
              v-model="donnees.theme_phare_autre"
              libelle="Précisez le thème phare"
              :maxlength="200"
              obligatoire
            />
          </div>
        </template>

        <!-- ─── Étape 2 : fichiers et provenance ─── -->
        <template v-else-if="etapeCourante === 1">
          <!-- Média : fichier OU lien, jamais les deux (FR-056) -->
          <div class="flex flex-col gap-2">
            <p class="text-[14px]/[1.4] text-af-atone italic">
              {{ estVideo ? 'Vidéo' : estContenu ? 'Audio' : 'Flux de diffusion' }}
            </p>
            <input
              v-if="estContenu"
              type="file"
              :accept="estVideo ? 'video/*' : 'audio/*'"
              class="w-full text-[14px]/[1.4] text-af-corps file:mr-3 file:rounded-md file:border-0 file:bg-af-fond file:px-4 file:py-2 file:text-[14px] file:font-bold file:text-af-corps hover:file:bg-af-bordure"
              @change="surFichierMedia"
            >
            <input
              v-model="lienExterne"
              type="url"
              :disabled="!!fichierMedia"
              class="h-11 w-full rounded-md border border-af-bordure bg-af-surface px-4 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:border-af-chocolat focus:outline-none disabled:opacity-50"
              placeholder="…ou collez un lien (YouTube, flux en ligne…)"
            >
            <p v-if="fichierMedia" class="text-[12px] text-af-atone">
              Fichier choisi : {{ fichierMedia.name }}, le champ de lien est désactivé.
            </p>
          </div>

          <div class="flex flex-col gap-2">
            <p class="text-[14px]/[1.4] text-af-atone italic">Image de couverture</p>
            <input
              type="file"
              accept="image/*"
              class="w-full text-[14px]/[1.4] text-af-corps file:mr-3 file:rounded-md file:border-0 file:bg-af-fond file:px-4 file:py-2 file:text-[14px] file:font-bold file:text-af-corps hover:file:bg-af-bordure"
              @change="surFichierImage"
            >
          </div>

          <!-- Source et auteur : aucune décharge de droits n'est recueillie
               (H-012), l'administrateur se prononce seul sur la licéité. -->
          <div class="grid gap-5 sm:grid-cols-2">
            <AfricansChamp
              v-model="donnees.source_declaree"
              libelle="Source du média"
              :maxlength="300"
              placeholder="D'où provient ce contenu ?"
            />
            <AfricansChamp
              v-model="donnees.auteur_declare"
              libelle="Auteur du contenu"
              :maxlength="300"
              placeholder="Qui l'a réalisé ?"
            />
          </div>
        </template>

        <!-- ─── Étape 3 : contacts publics et justification ─── -->
        <template v-else>
          <!-- Coordonnées publiques du support (09p). Réservées aux chaînes et
               stations : une émission n'a pas d'équipe propre à joindre, c'est
               son support qui la porte. -->
          <div v-if="estSupport" class="flex flex-col gap-4 rounded-lg border border-af-bordure p-4">
            <div>
              <p class="text-[14px]/[1.4] font-bold text-af-encre">Contacts de votre média</p>
              <p class="mt-0.5 text-[12px]/[1.6] text-af-atone">
                Facultatif. Ces coordonnées seront affichées sur la page publique de votre
                {{ typeObjet === 'chaine_tv' ? 'chaîne' : 'station' }} une fois la proposition
                validée : n'y mettez que ce que vous acceptez de rendre public.
              </p>
            </div>

            <div class="grid gap-4 sm:grid-cols-2">
              <AfricansChamp v-model="donnees.contact_email" libelle="E-mail" type="email" :maxlength="320" placeholder="contact@votremedia.tv" />
              <AfricansChamp v-model="donnees.contact_telephone" libelle="Téléphone" type="tel" :maxlength="50" placeholder="+225 01 02 03 04 05" />
              <AfricansChamp v-model="donnees.contact_whatsapp" libelle="WhatsApp" type="tel" :maxlength="50" placeholder="+225 01 02 03 04 05" />
              <AfricansChamp v-model="donnees.contact_site_web" libelle="Site web" :maxlength="500" placeholder="www.votremedia.tv" />
            </div>

            <AfricansChamp v-model="donnees.contact_adresse" libelle="Adresse" :maxlength="300" placeholder="Siège, quartier, ville" />
          </div>

          <AfricansChamp
            v-model="justification"
            libelle="Pourquoi proposez-vous ce contenu ?"
            type="textarea"
            :lignes="3"
            placeholder="Ce mot accompagne votre proposition auprès de l'administrateur."
            obligatoire
          />
        </template>

        <p v-if="erreur" class="rounded-lg border border-af-live/20 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live">
          {{ erreur }}
        </p>
      </form>
    </template>

    <template v-if="!succes && userStore.accessToken" #actions>
      <button
        type="button"
        class="mr-auto text-base font-bold text-af-corps transition hover:opacity-70 disabled:opacity-50"
        :disabled="chargement"
        @click="fermer"
      >
        Annuler
      </button>
      <AfricansBouton
        v-if="etapeCourante > 0"
        variante="secondaire"
        icone="fa-solid fa-arrow-left"
        @click="etapeCourante -= 1"
      >
        Précédent
      </AfricansBouton>
      <AfricansBouton
        v-if="etapeCourante < ETAPES.length - 1"
        icone="fa-solid fa-arrow-right"
        @click="suivant"
      >
        Suivant
      </AfricansBouton>
      <AfricansBouton
        v-else
        type="submit"
        form="form-proposition-media"
        :desactive="chargement"
        :tourne="chargement"
        :icone="chargement ? 'fa-solid fa-spinner' : 'fa-solid fa-paper-plane'"
      >
        Soumettre
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>
