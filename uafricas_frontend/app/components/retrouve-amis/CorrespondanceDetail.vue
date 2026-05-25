<script setup lang="ts">
import { MOTIFS_SIGNALEMENT, type MotifSignalement, type CoordonneesChoix, type EtatCorrespondance } from '~/composables/useRetrouvAmis'

interface ResumeAnonyme {
  initiales: string
  ville?: string
  periode?: string
  criteres_communs: string[]
}

interface CorrespondanceDetailProps {
  id: string
  avis_id: string
  score: number
  details_score: Record<string, number | string>
  etat: EtatCorrespondance
  type_cible: string
  mon_role: string
  resume_anonymise: ResumeAnonyme
  coordonnees_partagees?: Record<string, any> | null
  message_reponse?: string
  type_reponse_publique?: string
  created_at: string
  expire_at?: string
}

const props = defineProps<{
  correspondance: CorrespondanceDetailProps
}>()

const emit = defineEmits<{
  accepter: [id: string, coordonnees: CoordonneesChoix]
  refuser: [id: string]
  signaler: [avisId: string, motif: MotifSignalement, description: string]
}>()

// Etat du formulaire d'acceptation
const afficherAcceptation = ref(false)
const coordonnees = reactive<CoordonneesChoix>({
  email: true,
  telephone: false,
  messagerie: false,
})

// Etat du formulaire de signalement
const afficherSignalement = ref(false)
const motifSignalement = ref<MotifSignalement>('contenu_abusif')
const descriptionSignalement = ref('')

// Labels des criteres de score
const labelsCriteres: Record<string, string> = {
  nom: 'Nom',
  ecole: 'Ecole',
  ville: 'Ville',
  pays: 'Territoire',
  periode: 'Periode',
  source: 'Source',
  type_reponse: 'Type de reponse',
  type_relation: 'Relation',
  localite: 'Localite',
  localite_rencontre: 'Localite',
  ecole_rencontre: 'Ecole',
  ville_rencontre: 'Ville',
  description: 'Description',
  description_physique: 'Physique',
  surnom: 'Surnom',
  genre: 'Genre',
  genre_recherche: 'Genre',
  comment_connu: 'Comment connu',
  prenom: 'Prenom',
  nom_recherche: 'Nom recherche',
}

/** Convertit une cle snake_case en label lisible */
const labelCritere = (cle: string): string => {
  if (labelsCriteres[cle]) return labelsCriteres[cle]
  return cle.replace(/_/g, ' ').replace(/^\w/, c => c.toUpperCase())
}

/** Labels lisibles pour les valeurs textuelles du score */
const labelsValeurs: Record<string, string> = {
  reponse_publique: 'Reponse publique',
  je_suis_cette_personne: 'Je suis cette personne',
  je_la_connais: 'Je la connais',
  jai_des_informations: 'J\'ai des informations',
  avis: 'Avis',
  profil: 'Profil',
}

/** Convertit une valeur snake_case en label lisible */
const labelValeur = (val: string | number): string => {
  if (typeof val === 'number') return `${val}%`
  if (labelsValeurs[val]) return labelsValeurs[val]
  return val.replace(/_/g, ' ').replace(/^\w/, c => c.toUpperCase())
}

// Couleur du score individuel
const couleurBarre = (valeur: number): string => {
  if (valeur >= 80) return 'bg-green-500'
  if (valeur >= 60) return 'bg-amber-500'
  if (valeur >= 40) return 'bg-orange-500'
  return 'bg-red-400'
}

// Classes CSS pour les etats
const etatClasses = computed(() => {
  const classes: Record<string, string> = {
    en_attente: 'bg-amber-100 text-amber-700 border-amber-200',
    acceptee_a: 'bg-blue-100 text-blue-700 border-blue-200',
    acceptee_b: 'bg-blue-100 text-blue-700 border-blue-200',
    mutuelle: 'bg-green-100 text-green-700 border-green-200',
    declinee: 'bg-red-100 text-red-700 border-red-200',
    archivee: 'bg-gray-100 text-gray-500 border-gray-200',
  }
  return classes[props.correspondance.etat] || 'bg-gray-100 text-gray-500 border-gray-200'
})

const etatLabel = computed(() => {
  const labels: Record<string, string> = {
    en_attente: 'En attente',
    acceptee_a: 'Acceptee (vous)',
    acceptee_b: 'Acceptee (autre partie)',
    mutuelle: 'Contact mutuel',
    declinee: 'Declinee',
    archivee: 'Archivee',
  }
  return labels[props.correspondance.etat] || props.correspondance.etat
})

// Timeline des etats
const etatsTimeline: { etat: EtatCorrespondance; label: string }[] = [
  { etat: 'en_attente', label: 'Creation' },
  { etat: 'acceptee_a', label: 'Acceptation A' },
  { etat: 'acceptee_b', label: 'Acceptation B' },
  { etat: 'mutuelle', label: 'Contact mutuel' },
]

const indexEtatActuel = computed(() => {
  if (props.correspondance.etat === 'declinee' || props.correspondance.etat === 'archivee') return -1
  return etatsTimeline.findIndex(e => e.etat === props.correspondance.etat)
})

// Peut-on agir sur cette correspondance ?
const peutAgir = computed(() => {
  const etat = props.correspondance.etat
  return etat === 'en_attente'
    || (etat === 'acceptee_a' && props.correspondance.mon_role !== 'auteur')
    || (etat === 'acceptee_b' && props.correspondance.mon_role === 'auteur')
})

// Au moins une coordonnee selectionnee
const coordonneesValides = computed(() => {
  return coordonnees.email || coordonnees.telephone || coordonnees.messagerie
})

// Actions
const onAccepter = () => {
  if (!coordonneesValides.value) return
  emit('accepter', props.correspondance.id, { ...coordonnees })
  afficherAcceptation.value = false
}

const onRefuser = () => {
  emit('refuser', props.correspondance.id)
}

const onSignaler = () => {
  emit('signaler', props.correspondance.avis_id, motifSignalement.value, descriptionSignalement.value)
  afficherSignalement.value = false
  descriptionSignalement.value = ''
}

// Formater une date
const formaterDate = (iso: string): string => {
  return new Date(iso).toLocaleDateString('fr-FR', { day: 'numeric', month: 'long', year: 'numeric' })
}
</script>

<template>
  <div class="bg-white rounded-2xl shadow-md border border-gray-100 overflow-hidden">
    <!-- En-tete -->
    <div class="bg-gradient-to-r from-amber-50 to-orange-50 p-6 border-b border-gray-100">
      <div class="flex flex-wrap items-center gap-4">
        <!-- Initiales -->
        <div class="w-16 h-16 rounded-full bg-amber-700 text-white font-bold flex items-center justify-center text-xl shadow-sm shrink-0">
          {{ correspondance.resume_anonymise.initiales }}
        </div>
        <div class="flex-1 min-w-0">
          <div class="flex flex-wrap items-center gap-2 mb-1">
            <RetrouveAmisScoreBadge :score="correspondance.score" taille="lg" />
            <span
              class="text-xs px-3 py-1 rounded-full font-medium border"
              :class="etatClasses"
            >
              {{ etatLabel }}
            </span>
            <span
              class="text-xs px-2 py-0.5 rounded-full font-medium"
              :class="correspondance.type_cible === 'avis'
                ? 'bg-indigo-50 text-indigo-700'
                : 'bg-violet-50 text-violet-700'"
            >
              {{ correspondance.type_cible === 'avis' ? 'Via avis' : 'Via profil' }}
            </span>
            <span
              class="text-xs px-2 py-0.5 rounded-full font-medium"
              :class="correspondance.mon_role === 'auteur'
                ? 'bg-amber-100 text-amber-700'
                : 'bg-sky-50 text-sky-700'"
            >
              {{ correspondance.mon_role === 'auteur' ? 'Vous etes l\'auteur' : 'Vous etes la cible' }}
            </span>
          </div>
          <p class="text-sm text-gray-500">
            Correspondance creee le {{ formaterDate(correspondance.created_at) }}
          </p>
        </div>
      </div>
    </div>

    <div class="p-6 space-y-8">
      <!-- Detail du score -->
      <section>
        <h3 class="text-lg font-semibold text-gray-800 mb-4">
          <font-awesome-icon :icon="['fas', 'chart-bar']" class="mr-2 text-amber-600" />
          Detail du score
        </h3>
        <div class="space-y-3">
          <div
            v-for="(valeur, critere) in correspondance.details_score"
            :key="critere"
            class="flex items-center gap-4"
          >
            <span class="text-sm text-gray-600 w-28 shrink-0">
              {{ labelCritere(critere as string) }}
            </span>
            <!-- Valeur numerique : barre de progression -->
            <template v-if="typeof valeur === 'number'">
              <div class="flex-1 h-3 bg-gray-100 rounded-full overflow-hidden">
                <div
                  class="h-full rounded-full transition-all duration-500"
                  :class="couleurBarre(valeur)"
                  :style="{ width: `${valeur}%` }"
                />
              </div>
              <span class="text-sm font-medium text-gray-700 w-12 text-right">{{ valeur }}%</span>
            </template>
            <!-- Valeur textuelle : badge lisible -->
            <template v-else>
              <span class="text-sm font-medium text-gray-700 px-3 py-1 bg-gray-100 rounded-full">
                {{ labelValeur(valeur) }}
              </span>
            </template>
          </div>
        </div>
      </section>

      <!-- Message de reponse publique -->
      <section v-if="correspondance.message_reponse">
        <h3 class="text-lg font-semibold text-gray-800 mb-4">
          <font-awesome-icon :icon="['fas', 'comment-dots']" class="mr-2 text-amber-600" />
          Message recu
        </h3>
        <div class="bg-blue-50 border border-blue-200 rounded-lg p-5">
          <div v-if="correspondance.type_reponse_publique" class="mb-3">
            <span class="text-xs px-3 py-1 rounded-full font-medium bg-blue-100 text-blue-700">
              {{ labelValeur(correspondance.type_reponse_publique) }}
            </span>
          </div>
          <p class="text-gray-700 leading-relaxed whitespace-pre-line">{{ correspondance.message_reponse }}</p>
        </div>
      </section>

      <!-- Timeline -->
      <section>
        <h3 class="text-lg font-semibold text-gray-800 mb-4">
          <font-awesome-icon :icon="['fas', 'timeline']" class="mr-2 text-amber-600" />
          Progression
        </h3>
        <div v-if="correspondance.etat === 'declinee' || correspondance.etat === 'archivee'" class="flex items-center gap-3 p-4 rounded-lg bg-gray-50 border border-gray-200">
          <font-awesome-icon
            :icon="['fas', correspondance.etat === 'declinee' ? 'xmark-circle' : 'archive']"
            class="text-lg"
            :class="correspondance.etat === 'declinee' ? 'text-red-500' : 'text-gray-400'"
          />
          <span class="text-sm text-gray-600">
            Cette correspondance a ete {{ correspondance.etat === 'declinee' ? 'declinee' : 'archivee' }}.
          </span>
        </div>
        <div v-else class="flex items-center gap-0">
          <template v-for="(etape, idx) in etatsTimeline" :key="etape.etat">
            <!-- Point -->
            <div class="flex flex-col items-center">
              <div
                class="w-8 h-8 rounded-full flex items-center justify-center text-xs font-bold border-2 transition-colors"
                :class="idx <= indexEtatActuel
                  ? 'bg-amber-700 border-amber-700 text-white'
                  : 'bg-white border-gray-300 text-gray-400'"
              >
                <font-awesome-icon
                  v-if="idx <= indexEtatActuel"
                  :icon="['fas', 'check']"
                  class="text-xs"
                />
                <span v-else>{{ idx + 1 }}</span>
              </div>
              <span
                class="text-xs mt-2 text-center max-w-20"
                :class="idx <= indexEtatActuel ? 'text-amber-700 font-medium' : 'text-gray-400'"
              >
                {{ etape.label }}
              </span>
            </div>
            <!-- Ligne entre les points -->
            <div
              v-if="idx < etatsTimeline.length - 1"
              class="flex-1 h-0.5 mb-6 mx-1"
              :class="idx < indexEtatActuel ? 'bg-amber-700' : 'bg-gray-200'"
            />
          </template>
        </div>
      </section>

      <!-- Resume anonymise -->
      <section v-if="correspondance.resume_anonymise.ville || correspondance.resume_anonymise.periode || correspondance.resume_anonymise.criteres_communs?.length">
        <h3 class="text-lg font-semibold text-gray-800 mb-4">
          <font-awesome-icon :icon="['fas', 'user-secret']" class="mr-2 text-amber-600" />
          Resume anonymise
        </h3>
        <div class="bg-gray-50 rounded-lg p-4 space-y-2">
          <p v-if="correspondance.resume_anonymise.ville" class="text-sm text-gray-600">
            <font-awesome-icon :icon="['fas', 'location-dot']" class="w-4 mr-2 text-green-600" />
            {{ correspondance.resume_anonymise.ville }}
          </p>
          <p v-if="correspondance.resume_anonymise.periode" class="text-sm text-gray-600">
            <font-awesome-icon :icon="['fas', 'calendar']" class="w-4 mr-2 text-amber-600" />
            {{ correspondance.resume_anonymise.periode }}
          </p>
          <div v-if="correspondance.resume_anonymise.criteres_communs?.length" class="flex flex-wrap gap-2 mt-2">
            <span
              v-for="c in correspondance.resume_anonymise.criteres_communs"
              :key="c"
              class="text-xs px-2.5 py-1 rounded-full bg-green-100 text-green-700 font-medium"
            >
              {{ c }}
            </span>
          </div>
        </div>
      </section>

      <!-- Coordonnees partagees (etat mutuelle) -->
      <section v-if="correspondance.etat === 'mutuelle' && correspondance.coordonnees_partagees">
        <h3 class="text-lg font-semibold text-green-700 mb-4">
          <font-awesome-icon :icon="['fas', 'address-book']" class="mr-2" />
          Coordonnees partagees
        </h3>
        <div class="bg-green-50 border border-green-200 rounded-lg p-5 space-y-3">
          <div v-if="correspondance.coordonnees_partagees.email" class="flex items-center gap-3">
            <font-awesome-icon :icon="['fas', 'envelope']" class="text-green-600 w-5" />
            <span class="text-sm text-gray-800">{{ correspondance.coordonnees_partagees.email }}</span>
          </div>
          <div v-if="correspondance.coordonnees_partagees.telephone" class="flex items-center gap-3">
            <font-awesome-icon :icon="['fas', 'phone']" class="text-green-600 w-5" />
            <span class="text-sm text-gray-800">{{ correspondance.coordonnees_partagees.telephone }}</span>
          </div>
          <div v-if="correspondance.coordonnees_partagees.messagerie" class="flex items-center gap-3">
            <font-awesome-icon :icon="['fas', 'comment-dots']" class="text-green-600 w-5" />
            <span class="text-sm text-gray-800">{{ correspondance.coordonnees_partagees.messagerie }}</span>
          </div>
        </div>
      </section>

      <!-- Actions : Accepter / Refuser -->
      <section v-if="peutAgir" class="border-t border-gray-100 pt-6">
        <div v-if="!afficherAcceptation" class="flex items-center gap-4">
          <button
            class="px-6 py-2.5 bg-green-600 text-white font-medium rounded-lg hover:bg-green-700 transition-colors cursor-pointer"
            @click="afficherAcceptation = true"
          >
            <font-awesome-icon :icon="['fas', 'handshake']" class="mr-2" />
            Accepter
          </button>
          <button
            class="px-6 py-2.5 bg-white text-red-600 font-medium rounded-lg border border-red-200 hover:bg-red-50 transition-colors cursor-pointer"
            @click="onRefuser"
          >
            <font-awesome-icon :icon="['fas', 'xmark']" class="mr-2" />
            Refuser
          </button>
        </div>

        <!-- Formulaire d'acceptation inline -->
        <div v-else class="bg-green-50 border border-green-200 rounded-lg p-5">
          <h4 class="text-sm font-semibold text-green-800 mb-3">
            Choisissez les coordonnees a partager :
          </h4>
          <div class="space-y-2 mb-4">
            <label class="flex items-center gap-3 cursor-pointer">
              <input
                v-model="coordonnees.email"
                type="checkbox"
                class="w-4 h-4 accent-green-600"
              />
              <font-awesome-icon :icon="['fas', 'envelope']" class="text-green-600 w-4" />
              <span class="text-sm text-gray-700">Adresse e-mail</span>
            </label>
            <label class="flex items-center gap-3 cursor-pointer">
              <input
                v-model="coordonnees.telephone"
                type="checkbox"
                class="w-4 h-4 accent-green-600"
              />
              <font-awesome-icon :icon="['fas', 'phone']" class="text-green-600 w-4" />
              <span class="text-sm text-gray-700">Telephone</span>
            </label>
            <label class="flex items-center gap-3 cursor-pointer">
              <input
                v-model="coordonnees.messagerie"
                type="checkbox"
                class="w-4 h-4 accent-green-600"
              />
              <font-awesome-icon :icon="['fas', 'comment-dots']" class="text-green-600 w-4" />
              <span class="text-sm text-gray-700">Messagerie interne</span>
            </label>
          </div>
          <p v-if="!coordonneesValides" class="text-xs text-red-500 mb-3">
            Veuillez selectionner au moins une coordonnee.
          </p>
          <div class="flex items-center gap-3">
            <button
              class="px-5 py-2 bg-green-600 text-white text-sm font-medium rounded-lg hover:bg-green-700 transition-colors cursor-pointer disabled:opacity-40 disabled:cursor-not-allowed"
              :disabled="!coordonneesValides"
              @click="onAccepter"
            >
              Confirmer l'acceptation
            </button>
            <button
              class="px-5 py-2 text-sm text-gray-600 hover:text-gray-800 transition-colors cursor-pointer"
              @click="afficherAcceptation = false"
            >
              Annuler
            </button>
          </div>
        </div>
      </section>

      <!-- Expiration -->
      <div v-if="correspondance.expire_at" class="flex items-center text-sm text-gray-400">
        <font-awesome-icon :icon="['fas', 'clock']" class="w-4 mr-2" />
        Expire le {{ formaterDate(correspondance.expire_at) }}
      </div>

      <!-- Signalement -->
      <section class="border-t border-gray-100 pt-4">
        <button
          v-if="!afficherSignalement"
          class="text-xs text-gray-400 hover:text-red-500 transition-colors cursor-pointer"
          @click="afficherSignalement = true"
        >
          <font-awesome-icon :icon="['fas', 'flag']" class="mr-1" />
          Signaler cet avis
        </button>

        <div v-else class="bg-red-50 border border-red-200 rounded-lg p-5 mt-2">
          <h4 class="text-sm font-semibold text-red-800 mb-3">Signaler cet avis</h4>
          <div class="space-y-3">
            <select
              v-model="motifSignalement"
              class="w-full px-3 py-2 bg-white border border-red-200 rounded-lg text-sm text-gray-700 focus:outline-none focus:ring-2 focus:ring-red-400 focus:border-red-400"
            >
              <option v-for="m in MOTIFS_SIGNALEMENT" :key="m.value" :value="m.value">
                {{ m.label }}
              </option>
            </select>
            <textarea
              v-model="descriptionSignalement"
              rows="3"
              placeholder="Description optionnelle..."
              class="w-full px-3 py-2 bg-white border border-red-200 rounded-lg text-sm text-gray-700 resize-none focus:outline-none focus:ring-2 focus:ring-red-400 focus:border-red-400"
            />
            <div class="flex items-center gap-3">
              <button
                class="px-4 py-2 bg-red-600 text-white text-sm font-medium rounded-lg hover:bg-red-700 transition-colors cursor-pointer"
                @click="onSignaler"
              >
                Envoyer le signalement
              </button>
              <button
                class="px-4 py-2 text-sm text-gray-600 hover:text-gray-800 transition-colors cursor-pointer"
                @click="afficherSignalement = false"
              >
                Annuler
              </button>
            </div>
          </div>
        </div>
      </section>
    </div>
  </div>
</template>
