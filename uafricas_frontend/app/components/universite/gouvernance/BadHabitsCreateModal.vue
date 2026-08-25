<template>
  <Transition name="modal-fade">
    <div
      v-if="open"
      class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/70 backdrop-blur-xs"
      @click.self="fermer"
    >
      <div
        class="relative w-full max-w-3xl bg-white shadow-2xl rounded-2xl max-h-[92vh] overflow-hidden flex flex-col"
        @click.stop
      >
        <div
          class="text-white p-6 flex items-center justify-between transition-colors duration-300"
          :class="estBonne
            ? 'bg-linear-to-r from-emerald-600 to-green-600'
            : 'bg-linear-to-r from-red-700 to-orange-600'"
        >
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-xl bg-white/20 flex items-center justify-center">
              <font-awesome-icon
                :icon="['fas', estBonne ? 'thumbs-up' : 'triangle-exclamation']"
                class="text-lg"
              />
            </div>
            <div>
              <h2 class="text-xl font-bold">
                {{ estBonne ? 'Goodhabits' : 'Badhabits' }}
              </h2>
              <p class="text-xs text-white/80">
                {{ estBonne
                  ? 'Valoriser une action exemplaire de gouvernance'
                  : 'Dénoncer un problème de gouvernance' }}
              </p>
            </div>
          </div>
          <button
            type="button"
            class="w-9 h-9 rounded-full hover:bg-white/20 flex items-center justify-center transition"
            @click="fermer"
          >
            <font-awesome-icon :icon="['fas', 'xmark']" />
          </button>
        </div>

        <form class="p-6 space-y-5 overflow-y-auto" @submit.prevent="soumettre">
          <div
            v-if="erreurMessage"
            class="bg-red-50 border-l-4 border-red-500 p-3 rounded-lg text-sm text-red-700 flex items-center gap-2"
          >
            <font-awesome-icon :icon="['fas', 'circle-exclamation']" />
            <span>{{ erreurMessage }}</span>
          </div>

          <!-- Toggle type de pratique -->
          <div class="grid grid-cols-2 gap-2 p-1 bg-gray-100 rounded-xl">
            <button
              type="button"
              class="flex items-center justify-center gap-2 py-2.5 rounded-lg text-sm font-semibold transition-all"
              :class="!estBonne
                ? 'bg-white text-red-700 shadow-sm'
                : 'text-gray-500 hover:text-gray-700'"
              @click="changerMode('mauvaise')"
            >
              <font-awesome-icon :icon="['fas', 'triangle-exclamation']" class="text-xs" />
              Badhabits
            </button>
            <button
              type="button"
              class="flex items-center justify-center gap-2 py-2.5 rounded-lg text-sm font-semibold transition-all"
              :class="estBonne
                ? 'bg-white text-emerald-700 shadow-sm'
                : 'text-gray-500 hover:text-gray-700'"
              @click="changerMode('bonne')"
            >
              <font-awesome-icon :icon="['fas', 'thumbs-up']" class="text-xs" />
              Goodhabits
            </button>
          </div>

          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              Titre <span class="text-red-500">*</span>
            </label>
            <input
              v-model="form.titre"
              type="text"
              required
              maxlength="350"
              :placeholder="estBonne
                ? 'Résumez la bonne pratique en une phrase...'
                : 'Résumez la mauvaise pratique en une phrase...'"
              class="w-full px-4 py-2.5 border border-gray-300 rounded-lg transition text-sm"
              :class="estBonne
                ? 'focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500'
                : 'focus:ring-2 focus:ring-red-500/30 focus:border-red-500'"
            />
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-2">
                Catégorie <span class="text-red-500">*</span>
              </label>
              <select
                v-model="form.categorie_probleme"
                required
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg transition text-sm bg-white"
                :class="estBonne
                  ? 'focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500'
                  : 'focus:ring-2 focus:ring-red-500/30 focus:border-red-500'"
              >
                <option
                  v-for="c in categoriesActives"
                  :key="c.value"
                  :value="c.value"
                >{{ c.label }}</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-2">
                {{ estBonne ? 'Impact' : 'Gravité' }}
              </label>
              <div class="flex gap-2">
                <button
                  v-for="n in niveauxActifs"
                  :key="n.value"
                  type="button"
                  class="flex-1 px-3 py-2.5 rounded-lg border-2 text-xs font-medium transition"
                  :class="niveauCourant === n.value
                    ? `${n.activeClass} border-current`
                    : 'bg-white border-gray-200 text-gray-600'"
                  @click="changerNiveau(n.value)"
                >
                  <font-awesome-icon :icon="n.icon" class="mr-1" />
                  {{ n.label }}
                </button>
              </div>
            </div>
          </div>

          <div v-if="form.categorie_probleme === 'autre'">
            <label class="block text-sm font-semibold text-gray-700 mb-2">Précisez la catégorie</label>
            <input
              v-model="form.categorie_probleme_detail"
              type="text"
              maxlength="200"
              class="w-full px-4 py-2.5 border border-gray-300 rounded-lg transition text-sm"
              :class="estBonne
                ? 'focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500'
                : 'focus:ring-2 focus:ring-red-500/30 focus:border-red-500'"
            />
          </div>

          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              Description générale <span class="text-red-500">*</span>
            </label>
            <textarea
              v-model="form.description_generale"
              rows="3"
              required
              :placeholder="estBonne
                ? 'Décrivez brièvement cette bonne pratique...'
                : 'Décrivez brièvement la problématique...'"
              class="w-full px-4 py-3 border border-gray-300 rounded-lg transition text-sm"
              :class="estBonne
                ? 'focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500'
                : 'focus:ring-2 focus:ring-red-500/30 focus:border-red-500'"
            />
          </div>

          <!-- Mauvaise pratique : détails (texte libre) -->
          <div v-if="!estBonne">
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              Détails de la problématique
              <span class="text-red-500">*</span>
            </label>
            <textarea
              v-model="form.details_problematique"
              rows="5"
              required
              placeholder="Expliquez en détail le problème, son contexte, ses conséquences..."
              class="w-full px-4 py-3 border border-gray-300 rounded-lg transition text-sm focus:ring-2 focus:ring-red-500/30 focus:border-red-500"
            />
          </div>

          <!-- Bonne pratique : modalités pratiques de reproductibilité (liste, 10 max) -->
          <div v-else>
            <div class="flex items-center justify-between mb-2">
              <label class="block text-sm font-semibold text-gray-700">
                Modalités pratiques de reproductibilité <span class="text-red-500">*</span>
                <span class="text-gray-400 font-normal">(10 modalités maximum)</span>
              </label>
              <button
                v-if="modalitesReproductibilite.length < 10"
                type="button"
                class="text-xs text-emerald-600 hover:text-emerald-700 font-medium"
                @click="ajouterModalite"
              >
                <font-awesome-icon :icon="['fas', 'plus']" class="mr-1" />
                Ajouter une modalité
              </button>
            </div>
            <p class="text-xs text-gray-400 mb-2">
              Décrivez les modalités pratiques pour reproduire cette bonne pratique : étapes, conditions, moyens...
            </p>
            <div v-if="modalitesReproductibilite.length === 0" class="text-xs text-gray-400 italic">
              Aucune modalité. Ajoutez au moins une modalité concrète.
            </div>
            <div v-for="(_, idx) in modalitesReproductibilite" :key="idx" class="flex items-center gap-2 mb-2">
              <span class="shrink-0 w-7 h-7 rounded-full bg-emerald-100 text-emerald-700 text-xs font-bold flex items-center justify-center">
                {{ idx + 1 }}
              </span>
              <input
                v-model="modalitesReproductibilite[idx]"
                type="text"
                maxlength="500"
                :placeholder="`Modalité ${idx + 1}...`"
                class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500 transition text-sm"
              >
              <button
                type="button"
                class="w-10 h-10 rounded-lg border border-gray-300 text-gray-500 hover:bg-gray-50 flex items-center justify-center transition"
                @click="retirerModalite(idx)"
              >
                <font-awesome-icon :icon="['fas', 'trash']" class="text-xs" />
              </button>
            </div>
          </div>

          <!-- Témoignages (texte) : distinct des preuves -->
          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">
              {{ estBonne ? 'Témoignages / Preuves' : 'Témoignages' }}
            </label>
            <textarea
              v-model="form.preuves_temoignages"
              rows="3"
              :placeholder="estBonne
                ? 'Témoignages, chiffres, médias...'
                : 'Témoignages de personnes affectées ou ayant constaté les faits...'"
              class="w-full px-4 py-3 border border-gray-300 rounded-lg transition text-sm"
              :class="estBonne
                ? 'focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500'
                : 'focus:ring-2 focus:ring-red-500/30 focus:border-red-500'"
            />
          </div>

          <!-- Bonne pratique : reproductibilité (texte) -->
          <div v-if="estBonne">
            <label class="block text-sm font-semibold text-gray-700 mb-2">Reproductibilité / Transposition</label>
            <textarea
              v-model="form.solutions_proposees"
              rows="3"
              placeholder="Comment d'autres communautés peuvent reproduire cette action ?"
              class="w-full px-4 py-3 border border-gray-300 rounded-lg transition text-sm focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500"
            />
          </div>

          <!-- Preuves (images ou PDF) : partagé Goodhabits / Badhabits -->
          <div>
            <label class="block text-sm font-semibold text-gray-700 mb-2">Preuves (images ou PDF)</label>
            <div class="flex flex-wrap items-center gap-3">
              <div
                v-for="(preuve, idx) in preuvesFichiers"
                :key="preuve.url"
                class="relative w-20 h-20 rounded-lg overflow-hidden border border-gray-200 bg-gray-50"
              >
                <img
                  v-if="preuve.type === 'image'"
                  :src="urlAbsolue(preuve.url)"
                  alt=""
                  class="w-full h-full object-cover"
                >
                <a
                  v-else
                  :href="urlAbsolue(preuve.url)"
                  target="_blank"
                  rel="noopener noreferrer"
                  class="w-full h-full flex flex-col items-center justify-center gap-1 text-red-500 hover:text-red-600 transition"
                >
                  <font-awesome-icon :icon="['fas', 'file-pdf']" class="text-2xl" />
                  <span class="text-[10px] font-semibold">PDF</span>
                </a>
                <button
                  type="button"
                  class="absolute top-0.5 right-0.5 w-5 h-5 rounded-full bg-black/60 text-white flex items-center justify-center hover:bg-black/80 transition"
                  @click="retirerPhoto(idx)"
                >
                  <font-awesome-icon :icon="['fas', 'xmark']" class="text-[10px]" />
                </button>
              </div>
              <label
                v-if="preuvesFichiers.length < 5"
                class="w-20 h-20 rounded-lg border-2 border-dashed border-gray-300 flex flex-col items-center justify-center text-gray-400 cursor-pointer transition text-xs gap-1"
                :class="[
                  estBonne ? 'hover:border-emerald-400 hover:text-emerald-500' : 'hover:border-red-400 hover:text-red-500',
                  { 'opacity-50 cursor-not-allowed': photoEnCours },
                ]"
              >
                <font-awesome-icon
                  :icon="photoEnCours ? ['fas', 'spinner'] : ['fas', 'paperclip']"
                  :class="{ 'animate-spin': photoEnCours }"
                />
                <span>Ajouter</span>
                <input
                  type="file"
                  accept="image/jpeg,image/png,image/webp,application/pdf"
                  multiple
                  class="sr-only"
                  :disabled="photoEnCours"
                  @change="onPhotosSelectionnees"
                >
              </label>
            </div>
            <p class="text-xs text-gray-400 mt-1">Jusqu'à 5 fichiers (images JPEG, PNG, WebP ou PDF).</p>
            <p v-if="erreurPhoto" class="text-xs text-red-600 mt-1">{{ erreurPhoto }}</p>
          </div>

          <!-- Mauvaise pratique : solutions proposées (liste, 10 max) -->
          <div v-if="!estBonne">
            <div class="flex items-center justify-between mb-2">
              <label class="block text-sm font-semibold text-gray-700">
                Solutions proposées <span class="text-gray-400 font-normal">(10 propositions maximum)</span>
              </label>
              <button
                v-if="solutions.length < 10"
                type="button"
                class="text-xs text-red-600 hover:text-red-700 font-medium"
                @click="ajouterSolution"
              >
                <font-awesome-icon :icon="['fas', 'plus']" class="mr-1" />
                Ajouter une proposition
              </button>
            </div>
            <div v-if="solutions.length === 0" class="text-xs text-gray-400 italic">
              Aucune proposition. Suggérez des solutions concrètes (facultatif).
            </div>
            <div v-for="(_, idx) in solutions" :key="idx" class="flex items-center gap-2 mb-2">
              <span class="shrink-0 w-7 h-7 rounded-full bg-red-100 text-red-700 text-xs font-bold flex items-center justify-center">
                {{ idx + 1 }}
              </span>
              <input
                v-model="solutions[idx]"
                type="text"
                maxlength="500"
                :placeholder="`Proposition ${idx + 1}...`"
                class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500/30 focus:border-red-500 transition text-sm"
              >
              <button
                type="button"
                class="w-10 h-10 rounded-lg border border-gray-300 text-gray-500 hover:bg-gray-50 flex items-center justify-center transition"
                @click="retirerSolution(idx)"
              >
                <font-awesome-icon :icon="['fas', 'trash']" class="text-xs" />
              </button>
            </div>
          </div>

          <!-- Mauvaise pratique : identité réelle obligatoire (jamais anonyme) -->
          <div v-if="!estBonne" class="bg-red-50 border border-red-200 rounded-lg p-4 space-y-3">
            <p class="text-xs font-semibold text-red-700 uppercase tracking-wide flex items-center gap-1.5">
              <font-awesome-icon :icon="['fas', 'id-card']" />
              Vos informations d'identité <span class="text-red-500">*</span>
            </p>
            <p class="text-xs text-red-700/80">
              Un signalement ne peut pas être anonyme : vous devez partager votre identité réelle et vos coordonnées.
            </p>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
              <input
                v-model="form.identite_nom"
                type="text"
                maxlength="150"
                placeholder="Nom (état civil) *"
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500/30 focus:border-red-500 transition text-sm bg-white"
              >
              <input
                v-model="form.identite_prenom"
                type="text"
                maxlength="150"
                placeholder="Prénom (état civil) *"
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500/30 focus:border-red-500 transition text-sm bg-white"
              >
              <input
                v-model="form.identite_courriel"
                type="email"
                maxlength="255"
                placeholder="Courriel *"
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500/30 focus:border-red-500 transition text-sm bg-white"
              >
              <input
                v-model="form.identite_contact"
                type="text"
                maxlength="50"
                placeholder="Contact (téléphone) *"
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg focus:ring-2 focus:ring-red-500/30 focus:border-red-500 transition text-sm bg-white"
              >
            </div>
          </div>

          <div class="bg-gray-50 rounded-lg p-4 space-y-3">
            <p class="text-xs font-semibold text-gray-500 uppercase tracking-wide">Localisation</p>
            <div>
              <label class="block text-sm font-semibold text-gray-700 mb-2">
                Territoire <span class="text-red-500">*</span>
              </label>
              <select
                v-model="form.pays_id"
                required
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg transition text-sm bg-white"
                :class="estBonne
                  ? 'focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500'
                  : 'focus:ring-2 focus:ring-red-500/30 focus:border-red-500'"
              >
                <option value="" disabled>Sélectionnez un territoire</option>
                <option v-for="p in paysListe" :key="p.id" :value="p.id">{{ p.nom }}</option>
              </select>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
              <input
                v-model="form.region"
                type="text"
                maxlength="250"
                placeholder="Région (facultatif)"
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg transition text-sm"
                :class="estBonne
                  ? 'focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500'
                  : 'focus:ring-2 focus:ring-red-500/30 focus:border-red-500'"
              />
              <input
                v-model="form.ville_quartier_zone"
                type="text"
                maxlength="350"
                placeholder="Ville / Quartier / Zone (facultatif)"
                class="w-full px-4 py-2.5 border border-gray-300 rounded-lg transition text-sm"
                :class="estBonne
                  ? 'focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500'
                  : 'focus:ring-2 focus:ring-red-500/30 focus:border-red-500'"
              />
            </div>
          </div>

          <div>
            <div class="flex items-center justify-between mb-2">
              <label class="block text-sm font-semibold text-gray-700">Médias (URLs facultatives)</label>
              <button
                v-if="mediasUrls.length < 5"
                type="button"
                class="text-xs font-medium transition"
                :class="estBonne
                  ? 'text-emerald-600 hover:text-emerald-700'
                  : 'text-red-600 hover:text-red-700'"
                @click="ajouterMedia"
              >
                <font-awesome-icon :icon="['fas', 'plus']" class="mr-1" />
                Ajouter une URL
              </button>
            </div>
            <div v-if="mediasUrls.length === 0" class="text-xs text-gray-400 italic">
              Aucun média. Ajoutez des URLs d'images ou vidéos si nécessaire.
            </div>
            <div v-for="(_, idx) in mediasUrls" :key="idx" class="flex gap-2 mb-2">
              <input
                v-model="mediasUrls[idx]"
                type="url"
                placeholder="https://..."
                class="flex-1 px-4 py-2 border border-gray-300 rounded-lg transition text-sm"
                :class="estBonne
                  ? 'focus:ring-2 focus:ring-emerald-500/30 focus:border-emerald-500'
                  : 'focus:ring-2 focus:ring-red-500/30 focus:border-red-500'"
              />
              <button
                type="button"
                class="w-10 h-10 rounded-lg border border-gray-300 text-gray-500 hover:bg-gray-50 flex items-center justify-center transition"
                @click="retirerMedia(idx)"
              >
                <font-awesome-icon :icon="['fas', 'trash']" class="text-xs" />
              </button>
            </div>
          </div>

          <label
            v-if="estBonne"
            class="flex items-center gap-3 p-3 rounded-lg border-2 cursor-pointer transition-all"
            :class="form.publication_anonyme
              ? 'border-emerald-500 bg-emerald-50 text-emerald-700'
              : 'border-gray-200 hover:border-gray-300 text-gray-600'"
          >
            <input v-model="form.publication_anonyme" type="checkbox" class="sr-only" />
            <span
              class="w-5 h-5 rounded-md border-2 flex items-center justify-center shrink-0"
              :class="form.publication_anonyme
                ? (estBonne ? 'border-emerald-500 bg-emerald-500' : 'border-red-500 bg-red-500')
                : 'border-gray-300'"
            >
              <font-awesome-icon v-if="form.publication_anonyme" :icon="['fas', 'check']" class="text-white text-xs" />
            </span>
            <div>
              <p class="text-sm font-medium">Publier de manière anonyme</p>
              <p class="text-xs opacity-75">Votre nom ne sera pas affiché publiquement</p>
            </div>
          </label>

          <div
            class="border rounded-lg p-3 flex items-start gap-2 text-xs"
            :class="estBonne
              ? 'bg-emerald-50 border-emerald-200 text-emerald-800'
              : 'bg-red-50 border-red-200 text-red-800'"
          >
            <font-awesome-icon :icon="['fas', 'circle-info']" class="mt-0.5" />
            <p>
              {{ estBonne
                ? 'Votre félicitation sera publiée immédiatement. Restez factuel et inspirant.'
                : 'Votre signalement sera publié immédiatement. Restez factuel et respectueux.' }}
            </p>
          </div>
        </form>

        <div class="border-t border-gray-100 p-4 flex items-center justify-end gap-3 bg-gray-50">
          <button
            type="button"
            class="px-5 py-2.5 rounded-lg border border-gray-200 text-gray-700 text-sm font-medium hover:bg-white transition"
            @click="fermer"
          >
            Annuler
          </button>
          <button
            type="button"
            :disabled="!estValide || enCours"
            class="px-5 py-2.5 rounded-lg text-white text-sm font-semibold disabled:opacity-50 disabled:cursor-not-allowed transition flex items-center gap-2 shadow-md"
            :class="estBonne
              ? 'bg-linear-to-r from-emerald-600 to-green-600 hover:from-emerald-700 hover:to-green-700'
              : 'bg-linear-to-r from-red-600 to-orange-600 hover:from-red-700 hover:to-orange-700'"
            @click="soumettre"
          >
            <font-awesome-icon v-if="enCours" :icon="['fas', 'spinner']" class="animate-spin" />
            <font-awesome-icon v-else :icon="['fas', 'paper-plane']" />
            {{ enCours ? 'Publication...' : 'Publier' }}
          </button>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import type { CreerBadHabitPayload, PaysPublic, TypePratique } from '~/composables/useGouvernance'
import type { TypePreuve } from '~/types/gouvernance'

interface Props {
  open: boolean
  typePratiqueInitial?: TypePratique
}

const props = withDefaults(defineProps<Props>(), {
  typePratiqueInitial: 'mauvaise',
})
const emit = defineEmits<{
  close: []
  created: [id: string]
}>()

const { creerBadHabit, getPays, uploaderPreuve } = useGouvernance()
const userStore = useUserStore()

const form = reactive<CreerBadHabitPayload>({
  type_pratique: props.typePratiqueInitial,
  titre: '',
  description_generale: '',
  details_problematique: '',
  categorie_probleme: 'autre',
  categorie_probleme_detail: undefined,
  gravite: props.typePratiqueInitial === 'mauvaise' ? 'faible' : undefined,
  impact: props.typePratiqueInitial === 'bonne' ? 'fort' : undefined,
  preuves_temoignages: undefined,
  solutions_proposees: undefined,
  reproductibilite: undefined,
  publication_anonyme: false,
  pays_id: '',
  region: undefined,
  ville_quartier_zone: undefined,
  identite_nom: undefined,
  identite_prenom: undefined,
  identite_courriel: undefined,
  identite_contact: undefined,
})

const mediasUrls = ref<string[]>([])
const paysListe = ref<PaysPublic[]>([])
const enCours = ref(false)
const erreurMessage = ref<string | null>(null)

// Solutions proposées (mauvaise pratique), liste de 10 propositions max
const solutions = ref<string[]>([])

// Modalités pratiques de reproductibilité (bonne pratique), liste de 10 modalités max
const modalitesReproductibilite = ref<string[]>([])

// Preuves (Goodhabits & Badhabits) : fichiers uploadés (images ou PDF), max 5
const preuvesFichiers = ref<{ url: string; type: TypePreuve }[]>([])
const photoEnCours = ref(false)
const erreurPhoto = ref<string | null>(null)

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string
const urlAbsolue = (url: string) => (url.startsWith('http') ? url : `${apiBase}${url}`)

const estBonne = computed(() => form.type_pratique === 'bonne')

/** Préremplit l'identité réelle depuis le compte connecté (signalement non anonyme). */
function prefillIdentite() {
  const u = userStore.user
  if (!u) return
  if (!form.identite_nom) form.identite_nom = u.nom
  if (!form.identite_prenom) form.identite_prenom = u.prenom
  if (!form.identite_courriel) form.identite_courriel = u.email
}

function ajouterSolution() {
  if (solutions.value.length < 10) solutions.value.push('')
}
function retirerSolution(idx: number) {
  solutions.value.splice(idx, 1)
}

function ajouterModalite() {
  if (modalitesReproductibilite.value.length < 10) modalitesReproductibilite.value.push('')
}
function retirerModalite(idx: number) {
  modalitesReproductibilite.value.splice(idx, 1)
}

/** Modalités non vides (bonne pratique), limitées à 10. */
const modalitesValides = computed(() =>
  modalitesReproductibilite.value.map(m => m.trim()).filter(m => m.length > 0).slice(0, 10),
)

async function onPhotosSelectionnees(evt: Event) {
  const input = evt.target as HTMLInputElement
  const fichiers = Array.from(input.files ?? [])
  if (!fichiers.length) return
  erreurPhoto.value = null
  photoEnCours.value = true
  try {
    for (const fichier of fichiers) {
      if (preuvesFichiers.value.length >= 5) break
      const { url, preuveType } = await uploaderPreuve(fichier)
      preuvesFichiers.value.push({ url, type: preuveType })
    }
  } catch (err) {
    erreurPhoto.value = err instanceof Error ? err.message : 'Téléversement impossible'
  } finally {
    photoEnCours.value = false
    input.value = ''
  }
}

function retirerPhoto(idx: number) {
  preuvesFichiers.value.splice(idx, 1)
}

const categoriesMauvaise = [
  { value: 'corruption' as const, label: 'Corruption' },
  { value: 'service_public_defaillant' as const, label: 'Service public défaillant' },
  { value: 'infrastructure_degradee' as const, label: 'Infrastructure dégradée' },
  { value: 'acces_services_limite' as const, label: 'Accès aux services limité' },
  { value: 'insalubrite' as const, label: 'Insalubrité' },
  { value: 'probleme_securite' as const, label: 'Problème de sécurité' },
  { value: 'autre' as const, label: 'Autre' },
]

const categoriesBonne = [
  { value: 'civisme' as const, label: 'Civisme' },
  { value: 'service_public_exemplaire' as const, label: 'Service public exemplaire' },
  { value: 'solidarite' as const, label: 'Solidarité' },
  { value: 'innovation_sociale' as const, label: 'Innovation sociale' },
  { value: 'initiative_citoyenne' as const, label: 'Initiative citoyenne' },
  { value: 'leadership_exemplaire' as const, label: 'Leadership exemplaire' },
  { value: 'transparence' as const, label: 'Transparence' },
  { value: 'environnement' as const, label: 'Environnement' },
  { value: 'education' as const, label: 'Éducation' },
  { value: 'sante' as const, label: 'Santé' },
  { value: 'autre' as const, label: 'Autre' },
]

const gravites = [
  { value: 'faible' as const, label: 'Faible', icon: ['fas', 'circle-info'], activeClass: 'bg-yellow-50 text-yellow-700' },
  { value: 'elevee' as const, label: 'Élevée', icon: ['fas', 'triangle-exclamation'], activeClass: 'bg-orange-50 text-orange-700' },
  { value: 'critique' as const, label: 'Critique', icon: ['fas', 'skull'], activeClass: 'bg-red-50 text-red-700' },
]

const impacts = [
  { value: 'faible' as const, label: 'Modeste', icon: ['fas', 'seedling'], activeClass: 'bg-lime-50 text-lime-700' },
  { value: 'fort' as const, label: 'Fort', icon: ['fas', 'leaf'], activeClass: 'bg-emerald-50 text-emerald-700' },
  { value: 'exemplaire' as const, label: 'Exemplaire', icon: ['fas', 'star'], activeClass: 'bg-green-50 text-green-700' },
]

const categoriesActives = computed(() => estBonne.value ? categoriesBonne : categoriesMauvaise)
const niveauxActifs = computed(() => estBonne.value ? impacts : gravites)
const niveauCourant = computed(() => estBonne.value ? form.impact : form.gravite)

function changerNiveau(valeur: string) {
  if (estBonne.value) {
    form.impact = valeur as 'faible' | 'fort' | 'exemplaire'
  } else {
    form.gravite = valeur as 'faible' | 'elevee' | 'critique'
  }
}

function changerMode(mode: TypePratique) {
  if (form.type_pratique === mode) return
  form.type_pratique = mode
  form.categorie_probleme = 'autre'
  if (mode === 'bonne') {
    form.gravite = undefined
    form.impact = 'fort'
    // Amorcer une première modalité de reproductibilité si la liste est vide
    if (modalitesReproductibilite.value.length === 0) modalitesReproductibilite.value = ['']
  } else {
    form.impact = undefined
    form.gravite = 'faible'
    // Signalement : jamais anonyme, identité réelle préremplie
    form.publication_anonyme = false
    prefillIdentite()
  }
}

const courrielValide = (c?: string) => !!c && c.includes('@') && c.includes('.')

const estValide = computed(() => {
  const base = form.titre.trim().length >= 5
    && form.description_generale.trim().length >= 10
    && !!form.categorie_probleme
    && !!form.pays_id
  if (estBonne.value) {
    // Bonne pratique : au moins une modalité de reproductibilité
    return base && modalitesValides.value.length > 0
  }
  // Mauvaise pratique : détails requis + identité réelle obligatoire
  return base
    && form.details_problematique.trim().length >= 10
    && !!form.identite_nom?.trim()
    && !!form.identite_prenom?.trim()
    && courrielValide(form.identite_courriel?.trim())
    && !!form.identite_contact?.trim()
})

function ajouterMedia() {
  if (mediasUrls.value.length < 5) mediasUrls.value.push('')
}

function retirerMedia(idx: number) {
  mediasUrls.value.splice(idx, 1)
}

function reinitialiser() {
  form.type_pratique = props.typePratiqueInitial
  form.titre = ''
  form.description_generale = ''
  form.details_problematique = ''
  form.categorie_probleme = 'autre'
  form.categorie_probleme_detail = undefined
  form.gravite = props.typePratiqueInitial === 'mauvaise' ? 'faible' : undefined
  form.impact = props.typePratiqueInitial === 'bonne' ? 'fort' : undefined
  form.preuves_temoignages = undefined
  form.solutions_proposees = undefined
  form.reproductibilite = undefined
  form.publication_anonyme = false
  form.pays_id = ''
  form.region = undefined
  form.ville_quartier_zone = undefined
  form.identite_nom = undefined
  form.identite_prenom = undefined
  form.identite_courriel = undefined
  form.identite_contact = undefined
  mediasUrls.value = []
  solutions.value = []
  modalitesReproductibilite.value = props.typePratiqueInitial === 'bonne' ? [''] : []
  preuvesFichiers.value = []
  erreurPhoto.value = null
  erreurMessage.value = null
}

function fermer() {
  if (enCours.value) return
  emit('close')
}

async function soumettre() {
  if (!estValide.value || enCours.value) return
  enCours.value = true
  erreurMessage.value = null
  try {
    // Bonne pratique : les modalités (liste) sont enregistrées dans `details_problematique`
    const details = estBonne.value
      ? modalitesValides.value.join('\n')
      : form.details_problematique.trim()

    const payload: CreerBadHabitPayload = {
      type_pratique: form.type_pratique,
      titre: form.titre.trim(),
      description_generale: form.description_generale.trim(),
      details_problematique: details,
      categorie_probleme: form.categorie_probleme,
      publication_anonyme: form.publication_anonyme,
      pays_id: form.pays_id,
    }
    if (estBonne.value) {
      if (form.impact) payload.impact = form.impact
      if (form.solutions_proposees?.trim()) payload.reproductibilite = form.solutions_proposees.trim()
    } else {
      // Mauvaise pratique : jamais anonyme + identité réelle obligatoire
      payload.publication_anonyme = false
      payload.identite_nom = form.identite_nom?.trim()
      payload.identite_prenom = form.identite_prenom?.trim()
      payload.identite_courriel = form.identite_courriel?.trim()
      payload.identite_contact = form.identite_contact?.trim()
      if (form.gravite) payload.gravite = form.gravite
      const sols = solutions.value.map(s => s.trim()).filter(s => s.length > 0).slice(0, 10)
      if (sols.length > 0) payload.solutions_propositions = sols
    }
    // Preuves (images ou PDF) : partagées Goodhabits / Badhabits
    if (preuvesFichiers.value.length > 0) {
      payload.preuves_photos = preuvesFichiers.value.map(p => p.url).slice(0, 5)
    }
    if (form.categorie_probleme === 'autre' && form.categorie_probleme_detail?.trim()) {
      payload.categorie_probleme_detail = form.categorie_probleme_detail.trim()
    }
    if (form.preuves_temoignages?.trim()) payload.preuves_temoignages = form.preuves_temoignages.trim()
    if (form.region?.trim()) payload.region = form.region.trim()
    if (form.ville_quartier_zone?.trim()) payload.ville_quartier_zone = form.ville_quartier_zone.trim()

    const urlsValides = mediasUrls.value.map(u => u.trim()).filter(u => u.length > 0)
    if (urlsValides.length > 0) payload.medias_urls = urlsValides

    const id = await creerBadHabit(payload)
    emit('created', id)
    reinitialiser()
  } catch (err) {
    erreurMessage.value = err instanceof Error ? err.message : 'Erreur lors de la publication'
  } finally {
    enCours.value = false
  }
}

watch(() => props.open, async (v) => {
  if (!v) {
    reinitialiser()
    return
  }
  if (!estBonne.value) prefillIdentite()
  else if (modalitesReproductibilite.value.length === 0) modalitesReproductibilite.value = ['']
  if (paysListe.value.length === 0) {
    try {
      paysListe.value = await getPays()
    } catch (err) {
      erreurMessage.value = err instanceof Error ? err.message : 'Erreur chargement territoires'
    }
  }
})

watch(() => props.typePratiqueInitial, (v) => {
  if (!props.open) form.type_pratique = v
})
</script>

<style scoped>
.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.25s ease;
}
.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}
</style>
