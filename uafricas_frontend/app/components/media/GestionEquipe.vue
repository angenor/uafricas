<script setup lang="ts">
/**
 * Saisie de l'équipe éditoriale d'un support ou d'un programme, feature 010.
 *
 * **Un seul composant, monté côté membre ET côté back-office**, précédent
 * assumé de `GestionEpisodes.vue` : les règles sont les mêmes des deux côtés,
 * seule l'autorité diffère, et la prop `base` choisit le préfixe d'URL. Écrit
 * en **Tailwind v4 pur** (Principe VI) puisqu'il est monté sur des pages
 * membres, où daisyUI est proscrit.
 *
 * L'enregistrement est un **remplacement intégral** : l'ordre du tableau fait
 * foi (FR-016), et une liste vide supprime toute l'équipe. Le formulaire n'a
 * donc ni identifiant de ligne à suivre, ni verbe de suppression à appeler.
 *
 * Deux règles qui ne se voient pas mais se paient cher si on les oublie :
 *  • la fonction est un **texte libre**, les suggestions proposent, la saisie
 *    décide (FR-015) ;
 *  • le contact **n'est jamais pré-rempli depuis le compte rattaché** : un
 *    rattachement ne doit pas transformer l'adresse d'un compte en donnée
 *    publique (D2).
 */
import type {
  BaseEquipe,
  MembreEquipeForm,
  TypePorteurEquipe,
} from '~/composables/useMediaEquipe'
import { membreVideEquipe, versFormulaireEquipe } from '~/composables/useMediaEquipe'
import type { MembreAPI } from '~/composables/useMembres'

const props = withDefaults(defineProps<{
  typePorteur: TypePorteurEquipe
  porteurId: string
  base?: BaseEquipe
  /** Titre du bloc : le back-office l'intègre parfois à sa propre section. */
  titre?: string
  /** Le rendu s'adapte au fond de la page qui l'accueille. */
  sombre?: boolean
}>(), {
  base: 'membre',
  titre: 'Équipe éditoriale',
  sombre: false,
})

const emit = defineEmits<{ maj: [nombre: number] }>()

const { chargement, erreur, obtenirEquipe, definirEquipe, listerFonctions } = useMediaEquipe()
const { listerMembres } = useMembres()

const membres = ref<MembreEquipeForm[]>([])
const fonctions = ref<string[]>([])
const enregistrement = ref(false)
const confirmation = ref<string | null>(null)

const charger = async () => {
  const servis = await obtenirEquipe(props.typePorteur, props.porteurId, props.base)
  membres.value = servis.map(versFormulaireEquipe)
  // Le libellé du compte rattaché n'est pas servi par l'API d'équipe : on
  // l'affiche par son identifiant tant que l'utilisateur ne l'a pas re-choisi.
  membres.value.forEach((m, i) => {
    if (servis[i]?.utilisateur_id) m.compte_libelle = 'Compte rattaché'
  })
}

onMounted(async () => {
  await charger()
  fonctions.value = await listerFonctions()
})

// Changer de porteur (passer d'un programme à un autre dans le même panneau)
// doit recharger : sans cela, l'équipe du précédent serait enregistrée sur le
// suivant au premier clic.
//
// La source surveillée est une **chaîne**, pas un tableau littéral : un getter
// qui construit un nouveau tableau à chaque évaluation n'est jamais égal au
// précédent, si bien que le watcher se déclenchait à chaque invalidation
// réactive : donc à chaque frappe : et rechargeait par-dessus la saisie en
// cours. Le formulaire se vidait sous les doigts.
watch(() => `${props.typePorteur}:${props.porteurId}`, charger)

const ajouter = () => {
  membres.value.push(membreVideEquipe())
  confirmation.value = null
}

const retirer = (index: number) => {
  membres.value.splice(index, 1)
  confirmation.value = null
}

const deplacer = (index: number, sens: -1 | 1) => {
  const cible = index + sens
  if (cible < 0 || cible >= membres.value.length) return
  const [ligne] = membres.value.splice(index, 1)
  membres.value.splice(cible, 0, ligne as MembreEquipeForm)
  confirmation.value = null
}

/** Les deux champs obligatoires (FR-012), le serveur les revalide. */
const lignesIncompletes = computed(() =>
  membres.value.some(m => !m.nom.trim() || !m.fonction.trim()),
)

const enregistrer = async () => {
  if (lignesIncompletes.value) return
  enregistrement.value = true
  confirmation.value = null
  const resultat = await definirEquipe(
    props.typePorteur,
    props.porteurId,
    membres.value,
    props.base,
  )
  enregistrement.value = false
  if (!resultat) return

  membres.value = resultat.map(versFormulaireEquipe)
  // Le référentiel de fonctions vient de s'enrichir : le recharger rend la
  // nouveauté disponible dès la saisie suivante, sans rechargement de page.
  fonctions.value = await listerFonctions()
  confirmation.value = resultat.length
    ? `Équipe enregistrée, ${resultat.length} personne${resultat.length > 1 ? 's' : ''}.`
    : 'Équipe vidée.'
  emit('maj', resultat.length)
}

// ── Rattachement facultatif à un compte (FR-013, FR-014) ──────────────

const indexRecherche = ref<number | null>(null)
const requeteCompte = ref('')
const resultatsCompte = ref<MembreAPI[]>([])
const rechercheEnCours = ref(false)

const ouvrirRecherche = (index: number) => {
  indexRecherche.value = indexRecherche.value === index ? null : index
  requeteCompte.value = ''
  resultatsCompte.value = []
}

const chercherCompte = async () => {
  const requete = requeteCompte.value.trim()
  if (requete.length < 2) {
    resultatsCompte.value = []
    return
  }
  rechercheEnCours.value = true
  const liste = await listerMembres({ recherche: requete, par_page: 8 })
  rechercheEnCours.value = false
  resultatsCompte.value = liste?.membres ?? []
}

const rattacher = (index: number, compte: MembreAPI) => {
  const ligne = membres.value[index]
  if (!ligne) return
  ligne.utilisateur_id = compte.id
  ligne.compte_libelle = [compte.prenom, compte.nom].filter(Boolean).join(' ')
  // Le nom et le prénom sont PRÉ-REMPLIS s'ils sont vides : c'est du confort.
  // Le CONTACT, jamais : l'adresse du compte n'est pas une donnée publique (D2).
  if (!ligne.nom.trim()) ligne.nom = compte.nom
  if (!ligne.prenom.trim()) ligne.prenom = compte.prenom
  indexRecherche.value = null
  confirmation.value = null
}

const detacher = (index: number) => {
  const ligne = membres.value[index]
  if (!ligne) return
  ligne.utilisateur_id = null
  ligne.compte_libelle = undefined
  confirmation.value = null
}

const classeChamp = computed(() =>
  props.sombre
    ? 'w-full rounded-lg border border-white/15 bg-af-fond px-3 py-2 text-sm text-af-encre placeholder-gray-500 focus:border-af-chocolat focus:outline-none'
    : 'w-full rounded-lg border border-gray-300 bg-white px-3 py-2 text-sm text-gray-900 placeholder-gray-400 focus:border-af-chocolat focus:outline-none focus:ring-1 focus:ring-af-chocolat',
)
</script>

<template>
  <section :class="sombre ? 'text-gray-200' : 'text-gray-800'">
    <header class="mb-3 flex flex-wrap items-center justify-between gap-3">
      <div>
        <h3 class="font-oswald text-base uppercase tracking-wide" :class="sombre ? 'text-af-encre' : 'text-gray-900'">
          {{ titre }}
        </h3>
        <p class="mt-0.5 text-xs" :class="sombre ? 'text-af-corps' : 'text-af-atone'">
          Les personnes qui font ce contenu. Nom et fonction sont obligatoires ;
          territoire et contact restent facultatifs.
        </p>
      </div>
      <button
        type="button"
        class="inline-flex items-center gap-2 rounded-full border border-af-chocolat px-4 py-1.5 text-sm font-semibold text-af-chocolat transition-colors hover:bg-af-chocolat/10"
        @click="ajouter"
      >
        <font-awesome-icon :icon="['fas', 'plus']" class="h-3 w-3" />
        Ajouter une personne
      </button>
    </header>

    <p v-if="chargement && !membres.length" class="py-4 text-sm" :class="sombre ? 'text-af-corps' : 'text-af-atone'">
      Chargement…
    </p>

    <p
      v-if="!chargement && !membres.length"
      class="rounded-lg border border-dashed px-4 py-6 text-center text-sm"
      :class="sombre ? 'border-white/15 text-af-corps' : 'border-gray-300 text-af-atone'"
    >
      Aucune personne déclarée. Le bloc « équipe » n'apparaîtra pas sur les pages publiques.
    </p>

    <ul class="space-y-3">
      <li
        v-for="(membre, index) in membres"
        :key="index"
        class="rounded-lg border p-3"
        :class="sombre ? 'border-af-bordure bg-af-fond' : 'border-gray-200 bg-gray-50'"
      >
        <div class="mb-2 flex items-center justify-between gap-2">
          <span class="text-xs font-semibold" :class="sombre ? 'text-af-corps' : 'text-af-atone'">
            {{ index + 1 }}<sup v-if="index === 0">re</sup><sup v-else>e</sup> position
          </span>
          <div class="flex items-center gap-1">
            <button
              type="button"
              class="rounded p-1.5 transition-colors disabled:opacity-30"
              :class="sombre ? 'text-af-corps hover:opacity-70' : 'text-af-atone hover:text-af-chocolat'"
              :disabled="index === 0"
              title="Remonter"
              @click="deplacer(index, -1)"
            >
              <font-awesome-icon :icon="['fas', 'arrow-up']" class="h-3 w-3" />
            </button>
            <button
              type="button"
              class="rounded p-1.5 transition-colors disabled:opacity-30"
              :class="sombre ? 'text-af-corps hover:opacity-70' : 'text-af-atone hover:text-af-chocolat'"
              :disabled="index === membres.length - 1"
              title="Descendre"
              @click="deplacer(index, 1)"
            >
              <font-awesome-icon :icon="['fas', 'arrow-down']" class="h-3 w-3" />
            </button>
            <button
              type="button"
              class="rounded p-1.5 text-red-500 transition-colors hover:text-af-live"
              title="Retirer cette personne"
              @click="retirer(index)"
            >
              <font-awesome-icon :icon="['fas', 'trash']" class="h-3 w-3" />
            </button>
          </div>
        </div>

        <div class="grid gap-2 sm:grid-cols-2 lg:grid-cols-3">
          <label class="block">
            <span class="mb-1 block text-xs" :class="sombre ? 'text-af-corps' : 'text-af-atone-2'">
              Nom <span class="text-red-500">*</span>
            </span>
            <input v-model="membre.nom" type="text" placeholder="Diallo" :class="classeChamp">
          </label>

          <label class="block">
            <span class="mb-1 block text-xs" :class="sombre ? 'text-af-corps' : 'text-af-atone-2'">Prénom</span>
            <input v-model="membre.prenom" type="text" placeholder="Aminata" :class="classeChamp">
          </label>

          <div class="block">
            <span class="mb-1 block text-xs" :class="sombre ? 'text-af-corps' : 'text-af-atone-2'">
              Fonction <span class="text-red-500">*</span>
            </span>
            <!-- Texte libre assisté : toute fonction nouvelle est acceptée et
                 rejoint le référentiel dès l'enregistrement (FR-015). -->
            <CommonChampCombo
              v-model="membre.fonction"
              :suggestions="fonctions"
              placeholder="Directrice des programmes"
              aria-label="Fonction"
            />
          </div>

          <label class="block">
            <span class="mb-1 block text-xs" :class="sombre ? 'text-af-corps' : 'text-af-atone-2'">Territoire</span>
            <input v-model="membre.territoire" type="text" placeholder="Sénégal" :class="classeChamp">
          </label>

          <label class="block sm:col-span-2">
            <span class="mb-1 block text-xs" :class="sombre ? 'text-af-corps' : 'text-af-atone-2'">
              Contact professionnel
            </span>
            <input
              v-model="membre.contact"
              type="text"
              placeholder="redaction@exemple.org"
              :class="classeChamp"
            >
          </label>
        </div>

        <!-- Rattachement facultatif à un compte : il rend le nom cliquable vers
             le profil public, et RIEN d'autre, aucun droit n'en découle. -->
        <div class="mt-2 border-t pt-2" :class="sombre ? 'border-af-bordure' : 'border-gray-200'">
          <div class="flex flex-wrap items-center gap-2 text-xs">
            <span :class="sombre ? 'text-af-corps' : 'text-af-atone'">Compte UAfricas :</span>
            <span v-if="membre.utilisateur_id" class="font-medium text-af-vert">
              {{ membre.compte_libelle || 'rattaché' }}
            </span>
            <span v-else :class="sombre ? 'text-af-atone' : 'text-af-corps'">non rattaché</span>

            <button
              type="button"
              class="text-af-chocolat underline underline-offset-2 hover:opacity-80"
              @click="ouvrirRecherche(index)"
            >
              {{ indexRecherche === index ? 'fermer' : 'rechercher' }}
            </button>
            <button
              v-if="membre.utilisateur_id"
              type="button"
              class="text-red-500 underline underline-offset-2 hover:opacity-80"
              @click="detacher(index)"
            >
              détacher
            </button>
          </div>

          <div v-if="indexRecherche === index" class="mt-2">
            <div class="flex gap-2">
              <input
                v-model="requeteCompte"
                type="text"
                placeholder="Nom, prénom ou e-mail"
                :class="classeChamp"
                @keyup.enter="chercherCompte"
              >
              <button
                type="button"
                class="shrink-0 rounded-lg bg-af-chocolat px-3 py-2 text-sm text-af-encre transition-colors hover:opacity-90"
                @click="chercherCompte"
              >
                Chercher
              </button>
            </div>
            <p v-if="rechercheEnCours" class="mt-1 text-xs" :class="sombre ? 'text-af-corps' : 'text-af-atone'">
              Recherche…
            </p>
            <ul v-else-if="resultatsCompte.length" class="mt-2 space-y-1">
              <li v-for="compte in resultatsCompte" :key="compte.id">
                <button
                  type="button"
                  class="w-full rounded px-2 py-1.5 text-left text-sm transition-colors"
                  :class="sombre ? 'text-gray-200 hover:bg-af-fond' : 'text-gray-700 hover:bg-white'"
                  @click="rattacher(index, compte)"
                >
                  {{ compte.prenom }} {{ compte.nom }}
                  <span v-if="compte.pays" class="text-xs" :class="sombre ? 'text-af-atone' : 'text-af-corps'">
                    · {{ compte.pays }}
                  </span>
                </button>
              </li>
            </ul>
            <p
              v-else-if="requeteCompte.trim().length >= 2"
              class="mt-1 text-xs"
              :class="sombre ? 'text-af-atone' : 'text-af-corps'"
            >
              Aucun compte trouvé. Ce n'est pas bloquant : une fiche sans compte est parfaitement valide.
            </p>
          </div>
        </div>
      </li>
    </ul>

    <p v-if="erreur" class="mt-3 text-sm text-red-500">{{ erreur }}</p>
    <p v-if="confirmation" class="mt-3 text-sm text-af-vert">{{ confirmation }}</p>
    <p v-if="lignesIncompletes" class="mt-3 text-sm text-amber-500">
      Chaque personne doit au moins porter un nom et une fonction.
    </p>

    <div class="mt-4 flex items-center gap-3">
      <button
        type="button"
        class="rounded-full bg-af-chocolat px-5 py-2 text-sm font-semibold text-af-encre transition-colors hover:opacity-90 disabled:cursor-not-allowed disabled:opacity-50"
        :disabled="enregistrement || lignesIncompletes"
        @click="enregistrer"
      >
        {{ enregistrement ? 'Enregistrement…' : 'Enregistrer l’équipe' }}
      </button>
      <button
        type="button"
        class="text-sm underline underline-offset-2 transition-colors"
        :class="sombre ? 'text-af-corps hover:opacity-70' : 'text-af-atone hover:text-af-chocolat'"
        :disabled="enregistrement"
        @click="charger"
      >
        Annuler mes modifications
      </button>
    </div>
  </section>
</template>
