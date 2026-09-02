<template>
  <form class="flex flex-col gap-6" @submit.prevent="soumettre">
    <AfricansEtapes :etapes="ETAPES" :courante="etapeCourante" @aller="etapeCourante = $event" />

    <!-- `v-show` et non `v-if` : l'étape des photos porte un champ de fichier,
         qui perdrait l'affichage du fichier choisi à chaque démontage. -->
    <div v-show="etapeCourante === 0" class="flex flex-col gap-6">
      <!-- Type d'opération -->
      <div>
        <label class="block text-sm font-semibold text-af-corps mb-2">Type d'annonce *</label>
        <div class="grid grid-cols-2 gap-3">
          <button
            v-for="t in typesEchange"
            :key="t.value"
            type="button"
            class="py-2.5 px-3 rounded-lg border text-sm font-medium transition-all"
            :class="form.typeEchange === t.value
              ? 'border-af-chocolat bg-af-chocolat/10 text-af-chocolat'
              : 'border-af-bordure text-af-corps hover:border-af-chocolat'"
            @click="form.typeEchange = t.value"
          >
            {{ t.label }}
          </button>
        </div>
      </div>

      <!-- Titre -->
      <div>
        <label class="block text-sm font-semibold text-af-corps mb-2" for="titre">Titre *</label>
        <input
          id="titre"
          v-model="form.titre"
          type="text"
          maxlength="350"
          class="w-full px-4 py-2.5 border border-af-bordure rounded-lg focus:outline-hidden focus:ring-2 focus:ring-af-vert"
          placeholder="Ex. : Vélo tout-terrain en bon état"
        />
      </div>

      <!-- Description -->
      <div>
        <label class="block text-sm font-semibold text-af-corps mb-2" for="description">Description *</label>
        <textarea
          id="description"
          v-model="form.description"
          rows="4"
          class="w-full px-4 py-2.5 border border-af-bordure rounded-lg focus:outline-hidden focus:ring-2 focus:ring-af-vert"
          placeholder="Décrivez l'article, son état, les détails utiles…"
        ></textarea>
      </div>

      <!-- Catégorie + Condition -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <label class="block text-sm font-semibold text-af-corps mb-2" for="categorie">Catégorie *</label>
          <select
            id="categorie"
            v-model="form.categorieId"
            class="w-full px-4 py-2.5 border border-af-bordure rounded-lg focus:outline-hidden focus:ring-2 focus:ring-af-vert bg-white"
          >
            <option value="" disabled>Choisir une catégorie</option>
            <option v-for="cat in categories" :key="cat.id" :value="cat.id">{{ cat.nom }}</option>
          </select>
        </div>
        <div>
          <label class="block text-sm font-semibold text-af-corps mb-2" for="condition">État de l'article</label>
          <select
            id="condition"
            v-model="form.conditionArticle"
            class="w-full px-4 py-2.5 border border-af-bordure rounded-lg focus:outline-hidden focus:ring-2 focus:ring-af-vert bg-white"
          >
            <option value="non_applicable">Non applicable</option>
            <option value="neuf">Neuf</option>
            <option value="occasion">Occasion</option>
            <option value="reconditionne">Reconditionné</option>
          </select>
        </div>
      </div>

      <!-- Secteur d'activité (facultatif) -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <div>
          <label class="block text-sm font-semibold text-af-corps mb-2" for="secteur">
            Secteur d'activité <span class="text-af-atone-2 font-normal">(facultatif)</span>
          </label>
          <select
            id="secteur"
            v-model="form.secteurId"
            class="w-full px-4 py-2.5 border border-af-bordure rounded-lg focus:outline-hidden focus:ring-2 focus:ring-af-vert bg-white"
          >
            <option value="">Aucun secteur</option>
            <option v-for="s in secteurs" :key="s.id" :value="s.id">{{ s.nom }}</option>
            <option value="autre">Autre (préciser)…</option>
          </select>
        </div>
        <div v-if="form.secteurId === 'autre'">
          <label class="block text-sm font-semibold text-af-corps mb-2" for="secteur-autre">Préciser le secteur</label>
          <input
            id="secteur-autre"
            v-model="form.secteurAutre"
            type="text"
            maxlength="200"
            class="w-full px-4 py-2.5 border border-af-bordure rounded-lg focus:outline-hidden focus:ring-2 focus:ring-af-vert"
            placeholder="Ex. : Mines, Tourisme…"
          />
        </div>
      </div>
    </div>

    <div v-show="etapeCourante === 1" class="flex flex-col gap-6">
      <!-- Prix (vente uniquement) -->
      <div v-if="form.typeEchange === 'Vente'" class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div class="md:col-span-1">
          <label class="block text-sm font-semibold text-af-corps mb-2" for="prix">Prix *</label>
          <input
            id="prix"
            v-model.number="form.prix"
            type="number"
            min="0"
            step="any"
            class="w-full px-4 py-2.5 border border-af-bordure rounded-lg focus:outline-hidden focus:ring-2 focus:ring-af-vert"
            placeholder="0"
          />
        </div>
        <div>
          <label class="block text-sm font-semibold text-af-corps mb-2" for="devise">Devise</label>
          <select
            id="devise"
            v-model="form.devise"
            class="w-full px-4 py-2.5 border border-af-bordure rounded-lg focus:outline-hidden focus:ring-2 focus:ring-af-vert bg-white"
          >
            <option v-for="d in devises" :key="d.value" :value="d.value">{{ d.label }} ({{ d.symbol }})</option>
          </select>
        </div>
        <div class="flex items-end">
          <label class="inline-flex items-center gap-2 text-sm text-af-corps pb-2.5 cursor-pointer">
            <input v-model="form.prixNegociable" type="checkbox" class="rounded border-af-bordure text-af-vert focus:ring-af-vert" />
            Prix négociable
          </label>
        </div>
      </div>

      <!-- Localisation + quantité -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div>
          <label class="block text-sm font-semibold text-af-corps mb-2" for="ville">Ville</label>
          <input
            id="ville"
            v-model="form.ville"
            type="text"
            class="w-full px-4 py-2.5 border border-af-bordure rounded-lg focus:outline-hidden focus:ring-2 focus:ring-af-vert"
            placeholder="Ex. : Dakar"
          />
        </div>
        <div class="md:col-span-2">
          <label class="block text-sm font-semibold text-af-corps mb-2" for="adresse">Adresse (facultative)</label>
          <input
            id="adresse"
            v-model="form.adresse"
            type="text"
            class="w-full px-4 py-2.5 border border-af-bordure rounded-lg focus:outline-hidden focus:ring-2 focus:ring-af-vert"
            placeholder="Quartier, point de repère…"
          />
        </div>
      </div>

      <div>
        <label class="block text-sm font-semibold text-af-corps mb-2" for="quantite">Quantité disponible</label>
        <input
          id="quantite"
          v-model.number="form.quantite"
          type="number"
          min="1"
          class="w-full md:w-40 px-4 py-2.5 border border-af-bordure rounded-lg focus:outline-hidden focus:ring-2 focus:ring-af-vert"
        />
      </div>

      <!-- Territoires ciblés -->
      <div>
        <label class="block text-sm font-semibold text-af-corps mb-2">Territoires ciblés</label>
        <div class="space-y-3 max-h-72 overflow-y-auto p-3 border border-af-bordure rounded-lg">
          <!-- Bloc Afrique -->
          <div>
            <div class="flex items-center justify-between mb-2">
              <p class="text-xs font-semibold uppercase tracking-wide text-af-vert">Afrique</p>
              <button
                type="button"
                class="text-xs font-medium text-af-vert hover:underline"
                @click="basculerTousAfrique"
              >
                {{ toutAfriqueSelectionne ? 'Tout désélectionner' : "Toute l'Afrique" }}
              </button>
            </div>
            <div class="flex flex-wrap gap-2">
              <label
                v-for="terr in territoiresAfrique"
                :key="terr.id"
                class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs cursor-pointer transition-all"
                :class="form.paysIds.includes(terr.id)
                  ? 'bg-af-vert text-white'
                  : 'bg-af-fond text-af-corps hover:bg-af-bordure'"
              >
                <input type="checkbox" class="hidden" :value="terr.id" v-model="form.paysIds" />
                {{ terr.nom }}
              </label>
            </div>
          </div>
          <!-- Bloc hors Afrique -->
          <div class="pt-2 border-t border-af-bordure">
            <div class="flex items-center justify-between mb-2">
              <p class="text-xs font-semibold uppercase tracking-wide text-af-atone">Hors Afrique</p>
              <button
                type="button"
                class="text-xs font-medium text-af-vert hover:underline"
                @click="basculerTousHorsAfrique"
              >
                {{ toutHorsAfriqueSelectionne ? 'Tout désélectionner' : 'Tout Hors Afrique' }}
              </button>
            </div>
            <div class="flex flex-wrap gap-2">
              <label
                v-for="terr in territoiresHorsAfrique"
                :key="terr.id"
                class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-full text-xs cursor-pointer transition-all"
                :class="form.paysIds.includes(terr.id)
                  ? 'bg-af-vert text-white'
                  : 'bg-af-fond text-af-corps hover:bg-af-bordure'"
              >
                <input type="checkbox" class="hidden" :value="terr.id" v-model="form.paysIds" />
                {{ terr.nom }}
              </label>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-show="etapeCourante === 2" class="flex flex-col gap-6">
      <!-- Photos existantes (mode édition) -->
      <div v-if="mode === 'edition' && photosExistantes.length > 0">
        <label class="block text-sm font-semibold text-af-corps mb-2">Photos actuelles</label>
        <div class="flex flex-wrap gap-3">
          <div v-for="media in photosExistantes" :key="media.id" class="relative w-24 h-24 rounded-lg overflow-hidden border border-af-bordure">
            <img :src="media.media_url" :alt="form.titre" class="w-full h-full object-cover" />
            <span v-if="media.est_principale" class="absolute bottom-0 inset-x-0 bg-af-vert/90 text-white text-[10px] text-center py-0.5">Principale</span>
            <button
              type="button"
              class="absolute top-1 right-1 w-6 h-6 bg-af-live/90 text-white rounded-full flex items-center justify-center text-xs hover:bg-af-live"
              @click="retirerPhotoExistante(media.id)"
            >
              <font-awesome-icon :icon="['fas', 'xmark']" />
            </button>
          </div>
        </div>
      </div>

      <!-- Ajout de photos -->
      <div>
        <label class="block text-sm font-semibold text-af-corps mb-2">
          {{ mode === 'edition' ? 'Ajouter des photos' : 'Photos *' }}
          <span class="font-normal text-af-atone-2">(JPEG/PNG/WebP, max 3 Mo, {{ MAX_PHOTOS }} au total)</span>
        </label>
        <input
          ref="inputFichier"
          type="file"
          accept="image/jpeg,image/png,image/webp"
          multiple
          class="block w-full text-sm text-af-corps file:mr-4 file:py-2 file:px-4 file:rounded-lg file:border-0 file:bg-af-chocolat/10 file:text-af-chocolat hover:file:bg-af-chocolat/20"
          @change="onFichiersSelectionnes"
        />
        <div v-if="apercus.length > 0" class="flex flex-wrap gap-3 mt-3">
          <div v-for="(ap, i) in apercus" :key="i" class="relative w-24 h-24 rounded-lg overflow-hidden border border-af-bordure">
            <img :src="ap" alt="aperçu" class="w-full h-full object-cover" />
            <button
              type="button"
              class="absolute top-1 right-1 w-6 h-6 bg-af-live/90 text-white rounded-full flex items-center justify-center text-xs hover:bg-af-live"
              @click="retirerNouvellePhoto(i)"
            >
              <font-awesome-icon :icon="['fas', 'xmark']" />
            </button>
          </div>
        </div>
      </div>
    </div>

    <div v-show="etapeCourante === 3" class="flex flex-col gap-6">
      <!-- Type d'annonceur -->
      <div>
        <label class="block text-sm font-semibold text-af-corps mb-2">Vous publiez en tant que</label>
        <div class="grid grid-cols-2 gap-3">
          <label
            class="flex items-start gap-2 p-3 rounded-lg border cursor-pointer transition-all"
            :class="form.typeAnnonceur === 'particulier'
              ? 'border-af-vert bg-af-vert/5'
              : 'border-af-bordure hover:bg-af-fond'"
          >
            <input type="radio" value="particulier" v-model="form.typeAnnonceur" class="mt-0.5 accent-af-vert" />
            <span>
              <span class="block text-sm font-medium text-af-encre">En mon nom propre</span>
              <span class="block text-xs text-af-atone">Contact révélé sur demande</span>
            </span>
          </label>
          <label
            class="flex items-start gap-2 p-3 rounded-lg border cursor-pointer transition-all"
            :class="form.typeAnnonceur === 'entreprise'
              ? 'border-af-vert bg-af-vert/5'
              : 'border-af-bordure hover:bg-af-fond'"
          >
            <input type="radio" value="entreprise" v-model="form.typeAnnonceur" class="mt-0.5 accent-af-vert" />
            <span>
              <span class="block text-sm font-medium text-af-encre">Au nom d'une entreprise</span>
              <span class="block text-xs text-af-atone">Coordonnées affichées publiquement</span>
            </span>
          </label>
        </div>
      </div>

      <!-- Coordonnées de l'entreprise (affichées publiquement) -->
      <div v-if="form.typeAnnonceur === 'entreprise'" class="space-y-3 p-4 rounded-lg border border-af-bordure bg-af-fond">
        <div>
          <label class="block text-sm font-semibold text-af-corps mb-1">Nom de l'entreprise <span class="text-af-live">*</span></label>
          <input
            v-model="form.nomEntreprise"
            type="text"
            maxlength="200"
            placeholder="Ex : Sahel Distribution SARL"
            class="w-full px-4 py-2.5 border border-af-bordure rounded-lg focus:outline-hidden focus:ring-2 focus:ring-af-vert"
          />
        </div>
        <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
          <div>
            <label class="block text-sm font-semibold text-af-corps mb-1">Téléphone</label>
            <input
              v-model="form.contactTelephone"
              type="tel"
              maxlength="30"
              placeholder="+221 ..."
              class="w-full px-4 py-2.5 border border-af-bordure rounded-lg focus:outline-hidden focus:ring-2 focus:ring-af-vert"
            />
          </div>
          <div>
            <label class="block text-sm font-semibold text-af-corps mb-1">E-mail</label>
            <input
              v-model="form.contactEmail"
              type="email"
              maxlength="255"
              placeholder="contact@entreprise.com"
              class="w-full px-4 py-2.5 border border-af-bordure rounded-lg focus:outline-hidden focus:ring-2 focus:ring-af-vert"
            />
          </div>
        </div>
        <div>
          <label class="block text-sm font-semibold text-af-corps mb-1">Adresse</label>
          <input
            v-model="form.contactAdresse"
            type="text"
            maxlength="300"
            placeholder="Adresse de l'entreprise"
            class="w-full px-4 py-2.5 border border-af-bordure rounded-lg focus:outline-hidden focus:ring-2 focus:ring-af-vert"
          />
        </div>
      </div>

      <!-- Site web ou page réseau social (facultatif) -->
      <div>
        <label class="block text-sm font-semibold text-af-corps mb-2">Site web ou page réseau social <span class="text-af-atone-2 font-normal">(facultatif)</span></label>
        <input
          v-model="form.siteWebUrl"
          type="url"
          maxlength="500"
          placeholder="https://..."
          class="w-full px-4 py-2.5 border border-af-bordure rounded-lg focus:outline-hidden focus:ring-2 focus:ring-af-vert"
        />
      </div>
    </div>

    <!-- Erreur -->
    <p v-if="erreurForm" class="rounded-lg border border-af-live/30 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live">
      {{ erreurForm }}
    </p>

    <!-- Actions -->
    <div class="flex flex-wrap items-center gap-4 border-t border-af-bordure pt-5">
      <button
        type="button"
        class="text-base font-bold text-af-corps transition hover:opacity-70"
        @click="$emit('cancel')"
      >
        Annuler
      </button>

      <AfricansBouton
        v-if="etapeCourante > 0"
        variante="secondaire"
        icone="fa-solid fa-arrow-left"
        class="ml-auto"
        @click="etapeCourante -= 1"
      >
        Précédent
      </AfricansBouton>

      <AfricansBouton
        v-if="etapeCourante < ETAPES.length - 1"
        icone="fa-solid fa-arrow-right"
        :class="etapeCourante === 0 && 'ml-auto'"
        @click="suivant"
      >
        Suivant
      </AfricansBouton>
      <AfricansBouton
        v-else
        type="submit"
        :desactive="enCours"
        :tourne="enCours"
        :icone="enCours ? 'fa-solid fa-spinner' : 'fa-solid fa-paper-plane'"
      >
        {{ mode === 'edition' ? 'Enregistrer' : 'Publier' }}
      </AfricansBouton>
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
  type SecteurAnnonceAPI,
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

const { listerCategories, listerSecteurs, listerTerritoires, creerAnnonce, modifierAnnonce, supprimerMedia, erreur } =
  useMarcheAfricain()

const categories = ref<CategorieAnnonceAPI[]>([])
const secteurs = ref<SecteurAnnonceAPI[]>([])
const territoires = ref<PaysAPI[]>([])
const territoiresAfrique = computed(() => territoires.value.filter(t => t.continent === 'Afrique'))
const territoiresHorsAfrique = computed(() => territoires.value.filter(t => t.continent !== 'Afrique'))

// Sélection rapide des territoires (un seul clic par bloc)
const toutAfriqueSelectionne = computed(
  () => territoiresAfrique.value.length > 0 && territoiresAfrique.value.every(t => form.paysIds.includes(t.id)),
)
const toutHorsAfriqueSelectionne = computed(
  () => territoiresHorsAfrique.value.length > 0 && territoiresHorsAfrique.value.every(t => form.paysIds.includes(t.id)),
)

const basculerBloc = (terrs: PaysAPI[], toutSelectionne: boolean) => {
  const ids = terrs.map(t => t.id)
  if (toutSelectionne) {
    form.paysIds = form.paysIds.filter(id => !ids.includes(id))
  } else {
    form.paysIds = Array.from(new Set([...form.paysIds, ...ids]))
  }
}

const basculerTousAfrique = () => basculerBloc(territoiresAfrique.value, toutAfriqueSelectionne.value)
const basculerTousHorsAfrique = () => basculerBloc(territoiresHorsAfrique.value, toutHorsAfriqueSelectionne.value)
const photosExistantes = ref<AnnonceMediaAPI[]>([])
const apercus = ref<string[]>([])
const nouvellesPhotos = ref<File[]>([])
const inputFichier = ref<HTMLInputElement | null>(null)
const enCours = ref(false)
const erreurForm = ref<string | null>(null)

const typeEchangeInitial = (): TypeEchange => {
  const t = props.annonce?.type_echange
  if (t === 'Vente' || t === 'Troc' || t === 'Don' || t === "Opportunité d'investissement") return t
  return 'Vente'
}

const form = reactive<CreerAnnonceForm>({
  titre: props.annonce?.titre ?? '',
  description: props.annonce?.description ?? '',
  typeEchange: typeEchangeInitial(),
  categorieId: '',
  secteurId: '',
  secteurAutre: '',
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

const ETAPES = [
  { titre: "L'annonce" },
  { titre: 'Prix & lieu' },
  { titre: 'Photos' },
  { titre: 'Vous' },
] as const
const etapeCourante = ref(0)

/**
 * Ce qui manque à une étape, ou null. SOURCE UNIQUE de la validation :
 * `valider()` la parcourt, et l'envoi ramène à l'étape fautive — un message
 * rendu sur une étape masquée est un message perdu.
 */
const manqueEtape = (i: number): string | null => {
  switch (i) {
    case 0:
      if (form.titre.trim().length < 3) return 'Le titre doit contenir au moins 3 caractères.'
      if (form.description.trim().length < 10) return 'La description doit contenir au moins 10 caractères.'
      if (!form.categorieId) return 'Veuillez choisir une catégorie.'
      return null
    case 1:
      if (form.typeEchange === 'Vente' && (!form.prix || form.prix <= 0)) {
        return 'Un prix supérieur à 0 est requis pour une vente.'
      }
      return null
    case 2:
      // En édition, les photos déjà en ligne suffisent.
      if (props.mode === 'creation' && nouvellesPhotos.value.length === 0) {
        return 'Au moins une photo est requise.'
      }
      return null
    case 3:
      if (form.typeAnnonceur === 'entreprise' && !form.nomEntreprise?.trim()) {
        return "Le nom de l'entreprise est requis."
      }
      if (form.siteWebUrl?.trim() && !/^https?:\/\//i.test(form.siteWebUrl.trim())) {
        return 'Le lien doit commencer par http:// ou https://.'
      }
      return null
    default:
      return null
  }
}

const suivant = () => {
  const manque = manqueEtape(etapeCourante.value)
  if (manque) {
    erreurForm.value = manque
    return
  }
  erreurForm.value = null
  etapeCourante.value = Math.min(etapeCourante.value + 1, ETAPES.length - 1)
}

const valider = (): boolean => {
  for (let i = 0; i < ETAPES.length; i++) {
    const manque = manqueEtape(i)
    if (manque) {
      erreurForm.value = manque
      etapeCourante.value = i
      return false
    }
  }
  erreurForm.value = null
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
  const [cats, secs, terrs] = await Promise.all([
    listerCategories(),
    listerSecteurs(),
    listerTerritoires(),
  ])
  categories.value = cats
  secteurs.value = secs
  territoires.value = terrs

  if (props.annonce) {
    photosExistantes.value = [...props.annonce.medias]
    // Pré-sélectionner la catégorie par son nom (le détail public expose le nom)
    const cat = cats.find(c => c.nom === props.annonce?.categorie)
    if (cat) form.categorieId = cat.id
    // Pré-sélectionner le secteur (référentiel via secteur_id, sinon « Autre »)
    if (props.annonce.secteur_id) {
      form.secteurId = props.annonce.secteur_id
    } else if (props.annonce.secteur_autre) {
      form.secteurId = 'autre'
      form.secteurAutre = props.annonce.secteur_autre
    }
    // Pré-sélectionner les territoires par leur nom
    form.paysIds = terrs.filter(t => props.annonce?.pays.includes(t.nom)).map(t => t.id)
  }
})
</script>
