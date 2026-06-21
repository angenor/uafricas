<template>
  <form class="space-y-6" @submit.prevent="soumettre">
    <!-- Type d'opération -->
    <div>
      <label class="block text-sm font-semibold text-gray-700 mb-2">Type d'annonce *</label>
      <div class="grid grid-cols-3 gap-3">
        <button
          v-for="t in typesEchange"
          :key="t.value"
          type="button"
          class="py-2.5 px-3 rounded-xl border text-sm font-medium transition-all"
          :class="form.typeEchange === t.value
            ? 'border-custom-chocolat bg-custom-chocolat/10 text-custom-chocolat'
            : 'border-gray-200 text-gray-600 hover:border-gray-300'"
          @click="form.typeEchange = t.value"
        >
          {{ t.label }}
        </button>
      </div>
    </div>

    <!-- Titre -->
    <div>
      <label class="block text-sm font-semibold text-gray-700 mb-2" for="titre">Titre *</label>
      <input
        id="titre"
        v-model="form.titre"
        type="text"
        maxlength="350"
        class="w-full px-4 py-2.5 border border-gray-300 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-custom-green"
        placeholder="Ex. : Vélo tout-terrain en bon état"
      />
    </div>

    <!-- Description -->
    <div>
      <label class="block text-sm font-semibold text-gray-700 mb-2" for="description">Description *</label>
      <textarea
        id="description"
        v-model="form.description"
        rows="4"
        class="w-full px-4 py-2.5 border border-gray-300 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-custom-green"
        placeholder="Décrivez l'article, son état, les détails utiles…"
      ></textarea>
    </div>

    <!-- Catégorie + Condition -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div>
        <label class="block text-sm font-semibold text-gray-700 mb-2" for="categorie">Catégorie *</label>
        <select
          id="categorie"
          v-model="form.categorieId"
          class="w-full px-4 py-2.5 border border-gray-300 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-custom-green bg-white"
        >
          <option value="" disabled>Choisir une catégorie</option>
          <option v-for="cat in categories" :key="cat.id" :value="cat.id">{{ cat.nom }}</option>
        </select>
      </div>
      <div>
        <label class="block text-sm font-semibold text-gray-700 mb-2" for="condition">État de l'article</label>
        <select
          id="condition"
          v-model="form.conditionArticle"
          class="w-full px-4 py-2.5 border border-gray-300 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-custom-green bg-white"
        >
          <option value="non_applicable">Non applicable</option>
          <option value="neuf">Neuf</option>
          <option value="occasion">Occasion</option>
          <option value="reconditionne">Reconditionné</option>
        </select>
      </div>
    </div>

    <!-- Prix (vente uniquement) -->
    <div v-if="form.typeEchange === 'Vente'" class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <div class="md:col-span-1">
        <label class="block text-sm font-semibold text-gray-700 mb-2" for="prix">Prix *</label>
        <input
          id="prix"
          v-model.number="form.prix"
          type="number"
          min="0"
          step="any"
          class="w-full px-4 py-2.5 border border-gray-300 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-custom-green"
          placeholder="0"
        />
      </div>
      <div>
        <label class="block text-sm font-semibold text-gray-700 mb-2" for="devise">Devise</label>
        <select
          id="devise"
          v-model="form.devise"
          class="w-full px-4 py-2.5 border border-gray-300 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-custom-green bg-white"
        >
          <option v-for="d in devises" :key="d.value" :value="d.value">{{ d.label }} ({{ d.symbol }})</option>
        </select>
      </div>
      <div class="flex items-end">
        <label class="inline-flex items-center gap-2 text-sm text-gray-700 pb-2.5 cursor-pointer">
          <input v-model="form.prixNegociable" type="checkbox" class="rounded border-gray-300 text-custom-green focus:ring-custom-green" />
          Prix négociable
        </label>
      </div>
    </div>

    <!-- Localisation + quantité -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <div>
        <label class="block text-sm font-semibold text-gray-700 mb-2" for="ville">Ville</label>
        <input
          id="ville"
          v-model="form.ville"
          type="text"
          class="w-full px-4 py-2.5 border border-gray-300 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-custom-green"
          placeholder="Ex. : Dakar"
        />
      </div>
      <div class="md:col-span-2">
        <label class="block text-sm font-semibold text-gray-700 mb-2" for="adresse">Adresse (facultative)</label>
        <input
          id="adresse"
          v-model="form.adresse"
          type="text"
          class="w-full px-4 py-2.5 border border-gray-300 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-custom-green"
          placeholder="Quartier, point de repère…"
        />
      </div>
    </div>

    <div>
      <label class="block text-sm font-semibold text-gray-700 mb-2" for="quantite">Quantité disponible</label>
      <input
        id="quantite"
        v-model.number="form.quantite"
        type="number"
        min="1"
        class="w-full md:w-40 px-4 py-2.5 border border-gray-300 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-custom-green"
      />
    </div>

    <!-- Type d'annonceur -->
    <div>
      <label class="block text-sm font-semibold text-gray-700 mb-2">Vous publiez en tant que</label>
      <div class="grid grid-cols-2 gap-3">
        <label
          class="flex items-start gap-2 p-3 rounded-xl border cursor-pointer transition-all"
          :class="form.typeAnnonceur === 'particulier'
            ? 'border-custom-green bg-custom-green/5'
            : 'border-gray-200 hover:bg-gray-50'"
        >
          <input type="radio" value="particulier" v-model="form.typeAnnonceur" class="mt-0.5 accent-custom-green" />
          <span>
            <span class="block text-sm font-medium text-gray-800">En mon nom propre</span>
            <span class="block text-xs text-gray-500">Contact révélé sur demande</span>
          </span>
        </label>
        <label
          class="flex items-start gap-2 p-3 rounded-xl border cursor-pointer transition-all"
          :class="form.typeAnnonceur === 'entreprise'
            ? 'border-custom-green bg-custom-green/5'
            : 'border-gray-200 hover:bg-gray-50'"
        >
          <input type="radio" value="entreprise" v-model="form.typeAnnonceur" class="mt-0.5 accent-custom-green" />
          <span>
            <span class="block text-sm font-medium text-gray-800">Au nom d'une entreprise</span>
            <span class="block text-xs text-gray-500">Coordonnées affichées publiquement</span>
          </span>
        </label>
      </div>
    </div>

    <!-- Coordonnées de l'entreprise (affichées publiquement) -->
    <div v-if="form.typeAnnonceur === 'entreprise'" class="space-y-3 p-4 rounded-xl border border-gray-200 bg-gray-50">
      <div>
        <label class="block text-sm font-semibold text-gray-700 mb-1">Nom de l'entreprise <span class="text-red-500">*</span></label>
        <input
          v-model="form.nomEntreprise"
          type="text"
          maxlength="200"
          placeholder="Ex : Sahel Distribution SARL"
          class="w-full px-4 py-2.5 border border-gray-300 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-custom-green"
        />
      </div>
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
        <div>
          <label class="block text-sm font-semibold text-gray-700 mb-1">Téléphone</label>
          <input
            v-model="form.contactTelephone"
            type="tel"
            maxlength="30"
            placeholder="+221 ..."
            class="w-full px-4 py-2.5 border border-gray-300 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-custom-green"
          />
        </div>
        <div>
          <label class="block text-sm font-semibold text-gray-700 mb-1">E-mail</label>
          <input
            v-model="form.contactEmail"
            type="email"
            maxlength="255"
            placeholder="contact@entreprise.com"
            class="w-full px-4 py-2.5 border border-gray-300 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-custom-green"
          />
        </div>
      </div>
      <div>
        <label class="block text-sm font-semibold text-gray-700 mb-1">Adresse</label>
        <input
          v-model="form.contactAdresse"
          type="text"
          maxlength="300"
          placeholder="Adresse de l'entreprise"
          class="w-full px-4 py-2.5 border border-gray-300 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-custom-green"
        />
      </div>
    </div>

    <!-- Site web ou page réseau social (facultatif) -->
    <div>
      <label class="block text-sm font-semibold text-gray-700 mb-2">Site web ou page réseau social <span class="text-gray-400 font-normal">(facultatif)</span></label>
      <input
        v-model="form.siteWebUrl"
        type="url"
        maxlength="500"
        placeholder="https://..."
        class="w-full px-4 py-2.5 border border-gray-300 rounded-xl focus:outline-hidden focus:ring-2 focus:ring-custom-green"
      />
    </div>

    <!-- Territoires ciblés -->
    <div>
      <label class="block text-sm font-semibold text-gray-700 mb-2">Territoires ciblés</label>
      <div class="space-y-3 max-h-72 overflow-y-auto p-3 border border-gray-200 rounded-xl">
        <!-- Bloc Afrique -->
        <div>
          <p class="text-xs font-semibold uppercase tracking-wide text-custom-green mb-2">Afrique</p>
          <div class="flex flex-wrap gap-2">
            <label
              v-for="terr in territoiresAfrique"
              :key="terr.id"
              class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs cursor-pointer transition-all"
              :class="form.paysIds.includes(terr.id)
                ? 'bg-custom-green text-white'
                : 'bg-gray-100 text-gray-600 hover:bg-gray-200'"
            >
              <input type="checkbox" class="hidden" :value="terr.id" v-model="form.paysIds" />
              {{ terr.nom }}
            </label>
          </div>
        </div>
        <!-- Bloc hors Afrique -->
        <div class="pt-2 border-t border-gray-100">
          <p class="text-xs font-semibold uppercase tracking-wide text-gray-500 mb-2">Hors Afrique</p>
          <div class="flex flex-wrap gap-2">
            <label
              v-for="terr in territoiresHorsAfrique"
              :key="terr.id"
              class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs cursor-pointer transition-all"
              :class="form.paysIds.includes(terr.id)
                ? 'bg-custom-green text-white'
                : 'bg-gray-100 text-gray-600 hover:bg-gray-200'"
            >
              <input type="checkbox" class="hidden" :value="terr.id" v-model="form.paysIds" />
              {{ terr.nom }}
            </label>
          </div>
        </div>
      </div>
    </div>

    <!-- Photos existantes (mode édition) -->
    <div v-if="mode === 'edition' && photosExistantes.length > 0">
      <label class="block text-sm font-semibold text-gray-700 mb-2">Photos actuelles</label>
      <div class="flex flex-wrap gap-3">
        <div v-for="media in photosExistantes" :key="media.id" class="relative w-24 h-24 rounded-xl overflow-hidden border border-gray-200">
          <img :src="media.media_url" :alt="form.titre" class="w-full h-full object-cover" />
          <span v-if="media.est_principale" class="absolute bottom-0 inset-x-0 bg-custom-green/90 text-white text-[10px] text-center py-0.5">Principale</span>
          <button
            type="button"
            class="absolute top-1 right-1 w-6 h-6 bg-red-500/90 text-white rounded-full flex items-center justify-center text-xs hover:bg-red-600"
            @click="retirerPhotoExistante(media.id)"
          >
            <font-awesome-icon :icon="['fas', 'xmark']" />
          </button>
        </div>
      </div>
    </div>

    <!-- Ajout de photos -->
    <div>
      <label class="block text-sm font-semibold text-gray-700 mb-2">
        {{ mode === 'edition' ? 'Ajouter des photos' : 'Photos *' }}
        <span class="font-normal text-gray-400">(JPEG/PNG/WebP, max 3 Mo, {{ MAX_PHOTOS }} au total)</span>
      </label>
      <input
        ref="inputFichier"
        type="file"
        accept="image/jpeg,image/png,image/webp"
        multiple
        class="block w-full text-sm text-gray-600 file:mr-4 file:py-2 file:px-4 file:rounded-lg file:border-0 file:bg-custom-chocolat/10 file:text-custom-chocolat hover:file:bg-custom-chocolat/20"
        @change="onFichiersSelectionnes"
      />
      <div v-if="apercus.length > 0" class="flex flex-wrap gap-3 mt-3">
        <div v-for="(ap, i) in apercus" :key="i" class="relative w-24 h-24 rounded-xl overflow-hidden border border-gray-200">
          <img :src="ap" alt="aperçu" class="w-full h-full object-cover" />
          <button
            type="button"
            class="absolute top-1 right-1 w-6 h-6 bg-red-500/90 text-white rounded-full flex items-center justify-center text-xs hover:bg-red-600"
            @click="retirerNouvellePhoto(i)"
          >
            <font-awesome-icon :icon="['fas', 'xmark']" />
          </button>
        </div>
      </div>
    </div>

    <!-- Erreur -->
    <p v-if="erreurForm" class="text-sm text-red-600 bg-red-50 border border-red-200 rounded-lg px-4 py-2">
      {{ erreurForm }}
    </p>

    <!-- Actions -->
    <div class="flex items-center justify-end gap-3 pt-2">
      <button
        type="button"
        class="px-5 py-2.5 rounded-xl border border-gray-300 text-gray-600 hover:bg-gray-50 transition-colors"
        @click="$emit('cancel')"
      >
        Annuler
      </button>
      <button
        type="submit"
        :disabled="enCours"
        class="px-6 py-2.5 rounded-xl bg-linear-to-r from-custom-chocolat to-amber-700 text-white font-semibold hover:from-amber-700 hover:to-custom-chocolat transition-all disabled:opacity-60 flex items-center gap-2"
      >
        <span v-if="enCours" class="animate-spin rounded-full h-4 w-4 border-2 border-white border-t-transparent"></span>
        {{ mode === 'edition' ? 'Enregistrer' : 'Publier' }}
      </button>
    </div>
  </form>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import {
  useMarcheAfricain,
  TYPES_ECHANGE,
  DEVISES,
  type AnnonceDetailAPI,
  type AnnonceMediaAPI,
  type CategorieAnnonceAPI,
  type PaysAPI,
  type CreerAnnonceForm,
  type TypeEchange,
  type Devise,
} from '~/composables/useMarcheAfricain'

const props = withDefaults(
  defineProps<{
    mode?: 'creation' | 'edition'
    annonce?: AnnonceDetailAPI | null
  }>(),
  { mode: 'creation', annonce: null },
)

const emit = defineEmits<{
  (e: 'success', detail: AnnonceDetailAPI): void
  (e: 'cancel'): void
}>()

const MAX_PHOTOS = 5
const TAILLE_MAX = 3 * 1024 * 1024
const TYPES_MIME = ['image/jpeg', 'image/png', 'image/webp']

const typesEchange = TYPES_ECHANGE
const devises = DEVISES

const { listerCategories, listerTerritoires, creerAnnonce, modifierAnnonce, supprimerMedia, erreur } =
  useMarcheAfricain()

const categories = ref<CategorieAnnonceAPI[]>([])
const territoires = ref<PaysAPI[]>([])
const territoiresAfrique = computed(() => territoires.value.filter(t => t.continent === 'Afrique'))
const territoiresHorsAfrique = computed(() => territoires.value.filter(t => t.continent !== 'Afrique'))
const photosExistantes = ref<AnnonceMediaAPI[]>([])
const apercus = ref<string[]>([])
const nouvellesPhotos = ref<File[]>([])
const inputFichier = ref<HTMLInputElement | null>(null)
const enCours = ref(false)
const erreurForm = ref<string | null>(null)

const typeEchangeInitial = (): TypeEchange => {
  const t = props.annonce?.type_echange
  if (t === 'Vente' || t === 'Troc' || t === 'Don') return t
  return 'Vente'
}

const form = reactive<CreerAnnonceForm>({
  titre: props.annonce?.titre ?? '',
  description: props.annonce?.description ?? '',
  typeEchange: typeEchangeInitial(),
  categorieId: '',
  conditionArticle: 'non_applicable',
  prix: props.annonce?.prix ?? null,
  devise: (props.annonce?.devise as Devise) ?? 'XOF',
  prixNegociable: props.annonce?.prix_negociable ?? false,
  ville: props.annonce?.ville ?? '',
  adresse: props.annonce?.adresse ?? '',
  longitude: props.annonce?.longitude ?? null,
  latitude: props.annonce?.latitude ?? null,
  quantite: props.annonce?.quantite ?? 1,
  paysIds: [],
  photos: [],
  typeAnnonceur: (props.annonce?.type_annonceur as 'particulier' | 'entreprise') ?? 'particulier',
  nomEntreprise: props.annonce?.nom_entreprise ?? '',
  contactTelephone: props.annonce?.contact_telephone ?? '',
  contactEmail: props.annonce?.contact_email ?? '',
  contactAdresse: props.annonce?.contact_adresse ?? '',
  siteWebUrl: props.annonce?.site_web_url ?? '',
})

const totalPhotos = computed(() => photosExistantes.value.length + nouvellesPhotos.value.length)

const onFichiersSelectionnes = (e: Event) => {
  erreurForm.value = null
  const cible = e.target as HTMLInputElement
  const fichiers = Array.from(cible.files ?? [])
  for (const f of fichiers) {
    if (!TYPES_MIME.includes(f.type)) {
      erreurForm.value = `Format non autorisé : ${f.name} (JPEG, PNG ou WebP uniquement).`
      continue
    }
    if (f.size > TAILLE_MAX) {
      erreurForm.value = `Photo trop volumineuse : ${f.name} (max 3 Mo).`
      continue
    }
    if (totalPhotos.value >= MAX_PHOTOS) {
      erreurForm.value = `Maximum ${MAX_PHOTOS} photos par annonce.`
      break
    }
    nouvellesPhotos.value.push(f)
    apercus.value.push(URL.createObjectURL(f))
  }
  if (inputFichier.value) inputFichier.value.value = ''
}

const retirerNouvellePhoto = (i: number) => {
  URL.revokeObjectURL(apercus.value[i]!)
  apercus.value.splice(i, 1)
  nouvellesPhotos.value.splice(i, 1)
}

const retirerPhotoExistante = async (mediaId: string) => {
  if (!props.annonce) return
  const ok = await supprimerMedia(props.annonce.id, mediaId)
  if (ok) {
    photosExistantes.value = photosExistantes.value.filter(m => m.id !== mediaId)
  } else {
    erreurForm.value = erreur.value || 'Impossible de retirer la photo.'
  }
}

const valider = (): boolean => {
  erreurForm.value = null
  if (form.titre.trim().length < 3) {
    erreurForm.value = 'Le titre doit contenir au moins 3 caractères.'
    return false
  }
  if (form.description.trim().length < 10) {
    erreurForm.value = 'La description doit contenir au moins 10 caractères.'
    return false
  }
  if (!form.categorieId) {
    erreurForm.value = 'Veuillez choisir une catégorie.'
    return false
  }
  if (form.typeEchange === 'Vente' && (!form.prix || form.prix <= 0)) {
    erreurForm.value = 'Un prix supérieur à 0 est requis pour une vente.'
    return false
  }
  if (props.mode === 'creation' && nouvellesPhotos.value.length === 0) {
    erreurForm.value = 'Au moins une photo est requise.'
    return false
  }
  if (form.typeAnnonceur === 'entreprise' && !form.nomEntreprise?.trim()) {
    erreurForm.value = "Le nom de l'entreprise est requis."
    return false
  }
  if (form.siteWebUrl?.trim() && !/^https?:\/\//i.test(form.siteWebUrl.trim())) {
    erreurForm.value = 'Le lien doit commencer par http:// ou https://.'
    return false
  }
  return true
}

const soumettre = async () => {
  if (!valider()) return
  enCours.value = true
  try {
    const charge: CreerAnnonceForm = { ...form, photos: nouvellesPhotos.value }
    const detail = props.mode === 'edition' && props.annonce
      ? await modifierAnnonce(props.annonce.id, charge)
      : await creerAnnonce(charge)
    if (detail) {
      emit('success', detail)
    } else {
      erreurForm.value = erreur.value || "Une erreur s'est produite."
    }
  } finally {
    enCours.value = false
  }
}

onMounted(async () => {
  const [cats, terrs] = await Promise.all([listerCategories(), listerTerritoires()])
  categories.value = cats
  territoires.value = terrs

  if (props.annonce) {
    photosExistantes.value = [...props.annonce.medias]
    // Pré-sélectionner la catégorie par son nom (le détail public expose le nom)
    const cat = cats.find(c => c.nom === props.annonce?.categorie)
    if (cat) form.categorieId = cat.id
    // Pré-sélectionner les territoires par leur nom
    form.paysIds = terrs.filter(t => props.annonce?.pays.includes(t.nom)).map(t => t.id)
  }
})
</script>
