<script setup lang="ts">
import { DOMAINES_AFRICANTIVES, PAYS_AFRICAINS } from '~/composables/useAfricantives'

interface Props {
  isOpen: boolean
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'submit', data: {
    titre: string
    description: string
    domaine: string
    domaine_autre: string
    pays: string
    ville: string
    site_web_url: string
    lien_reseau_social: string
    contact1_courriel: string
    contact1_telephone: string
    contact1_adresse: string
    contact2_courriel: string
    contact2_telephone: string
    contact2_adresse: string
    couvertureFile: File | null
  }): void
}>()

const form = reactive({
  titre: '',
  description: '',
  domaine: '',
  domaine_autre: '',
  pays: '',
  ville: '',
  site_web_url: '',
  lien_reseau_social: '',
  contact1_courriel: '',
  contact1_telephone: '',
  contact1_adresse: '',
  contact2_courriel: '',
  contact2_telephone: '',
  contact2_adresse: '',
  couvertureFile: null as File | null,
  couverturePreview: '' as string,
  loading: false,
  submitted: false,
  error: false,
  errorMessage: '',
})

const domaines = DOMAINES_AFRICANTIVES.filter(d => d.value !== '')
const pays = PAYS_AFRICAINS

const isFormValid = computed(() => {
  return form.titre.trim().length >= 5 && form.description.trim().length >= 20
})

const ETAPES = [
  { titre: "L'initiative" },
  { titre: 'Domaine, lieu & liens' },
  { titre: 'Contacts & image' },
] as const
const etapeCourante = ref(0)

function suivant() {
  // Les deux seuls champs obligatoires vivent à l'étape 1 : `isFormValid`
  // reste l'autorité, l'étape ne fait que dire où l'erreur se corrige.
  if (etapeCourante.value === 0 && !isFormValid.value) {
    form.error = true
    form.errorMessage = 'Le titre (5 caractères minimum) et la description (20 minimum) sont requis.'
    return
  }
  form.error = false
  etapeCourante.value = Math.min(etapeCourante.value + 1, ETAPES.length - 1)
}

const resetForm = () => {
  etapeCourante.value = 0
  form.titre = ''
  form.description = ''
  form.domaine = ''
  form.domaine_autre = ''
  form.pays = ''
  form.ville = ''
  form.site_web_url = ''
  form.lien_reseau_social = ''
  form.contact1_courriel = ''
  form.contact1_telephone = ''
  form.contact1_adresse = ''
  form.contact2_courriel = ''
  form.contact2_telephone = ''
  form.contact2_adresse = ''
  form.couvertureFile = null
  form.couverturePreview = ''
  form.loading = false
  form.submitted = false
  form.error = false
  form.errorMessage = ''
}

const handleFileChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  if (target.files && target.files[0]) {
    const file = target.files[0]

    // Verifier la taille (max 5 Mo)
    if (file.size > 5 * 1024 * 1024) {
      form.error = true
      form.errorMessage = 'L\'image ne doit pas dépasser 5 Mo.'
      return
    }

    // Verifier le type
    if (!file.type.startsWith('image/')) {
      form.error = true
      form.errorMessage = 'Veuillez sélectionner un fichier image valide.'
      return
    }

    form.couvertureFile = file
    form.error = false
    form.errorMessage = ''

    // Preview
    const reader = new FileReader()
    reader.onload = (e) => {
      form.couverturePreview = e.target?.result as string
    }
    reader.readAsDataURL(file)
  }
}

const removeImage = () => {
  form.couvertureFile = null
  form.couverturePreview = ''
}

const handleSubmit = async () => {
  form.error = false
  form.errorMessage = ''

  if (!form.titre.trim()) {
    form.error = true
    form.errorMessage = 'Le titre est requis.'
    return
  }

  if (form.titre.trim().length < 5) {
    form.error = true
    form.errorMessage = 'Le titre doit contenir au moins 5 caractères.'
    return
  }

  if (!form.description.trim()) {
    form.error = true
    form.errorMessage = 'La description est requise.'
    return
  }

  if (form.description.trim().length < 20) {
    form.error = true
    form.errorMessage = 'La description doit contenir au moins 20 caractères.'
    return
  }

  if (form.domaine === 'Autre' && !form.domaine_autre.trim()) {
    form.error = true
    form.errorMessage = 'Veuillez préciser le domaine d\'activité.'
    return
  }

  emit('submit', {
    titre: form.titre.trim(),
    description: form.description.trim(),
    domaine: form.domaine,
    domaine_autre: form.domaine === 'Autre' ? form.domaine_autre.trim() : '',
    pays: form.pays,
    ville: form.ville.trim(),
    site_web_url: form.site_web_url.trim(),
    lien_reseau_social: form.lien_reseau_social.trim(),
    contact1_courriel: form.contact1_courriel.trim(),
    contact1_telephone: form.contact1_telephone.trim(),
    contact1_adresse: form.contact1_adresse.trim(),
    contact2_courriel: form.contact2_courriel.trim(),
    contact2_telephone: form.contact2_telephone.trim(),
    contact2_adresse: form.contact2_adresse.trim(),
    couvertureFile: form.couvertureFile,
  })
}

// Exposer pour que le parent puisse piloter l'etat
defineExpose({
  setLoading: (val: boolean) => { form.loading = val },
  setError: (msg: string) => { form.error = true; form.errorMessage = msg; form.loading = false },
  setSuccess: () => {
    form.submitted = true
    form.loading = false
    setTimeout(() => {
      resetForm()
      emit('close')
    }, 2000)
  },
})

watch(() => props.isOpen, (isOpen) => {
  if (!isOpen) {
    resetForm()
  }
})
</script>

<template>
  <AfricansModale
    :model-value="isOpen"
    titre="Publier une initiative"
    sous-titre="Partagez votre initiative africaine avec la communauté"
    icone="fa-solid fa-lightbulb"
    taille="large"
    @update:model-value="emit('close')"
  >
    <AfricansEtapes :etapes="ETAPES" :courante="etapeCourante" class="mb-6" @aller="etapeCourante = $event" />

    <form id="form-initiative" class="flex flex-col gap-5" @submit.prevent="handleSubmit">
      <p
        v-if="form.submitted"
        class="flex items-center gap-3 rounded-lg border border-af-vert/20 bg-af-vert/5 px-4 py-3 text-[14px]/[1.4] font-bold text-af-vert"
      >
        <font-awesome-icon icon="fa-solid fa-circle-check" />
        Initiative publiée avec succès !
      </p>

      <p
        v-if="form.error"
        class="flex items-center gap-3 rounded-lg border border-af-live/20 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live"
      >
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" />
        {{ form.errorMessage }}
      </p>

      <!-- ─── Étape 1 : l'initiative ─── -->
      <template v-if="etapeCourante === 0">
        <div>
          <AfricansChamp
            v-model="form.titre"
            libelle="Titre de l'initiative"
            :maxlength="350"
            placeholder="Ex: Plateforme de micro-crédit pour les agricultrices"
            obligatoire
          />
          <p class="mt-1 text-[12px] text-af-atone-2">{{ form.titre.length }}/350 caractères (minimum 5)</p>
        </div>

        <div>
          <AfricansChamp
            v-model="form.description"
            libelle="Description"
            type="textarea"
            :lignes="5"
            placeholder="Décrivez votre initiative en détail : objectifs, impact, bénéficiaires…"
            obligatoire
          />
          <p class="mt-1 text-[12px] text-af-atone-2">{{ form.description.length }} caractères (minimum 20)</p>
        </div>
      </template>

      <!-- ─── Étape 2 : domaine, localisation et liens ─── -->
      <template v-else-if="etapeCourante === 1">
        <AfricansChamp v-model="form.domaine" libelle="Domaine d'activité" type="select">
          <option value="">Sélectionnez un domaine (optionnel)</option>
          <option v-for="dom in domaines" :key="dom.value" :value="dom.value">{{ dom.label }}</option>
        </AfricansChamp>

        <AfricansChamp
          v-if="form.domaine === 'Autre'"
          v-model="form.domaine_autre"
          libelle="Précisez le domaine"
          :maxlength="200"
          placeholder="Ex: Diplomatie panafricaine"
          obligatoire
        />

        <div class="grid gap-5 md:grid-cols-2">
          <AfricansChamp v-model="form.pays" libelle="Territoire" type="select">
            <option value="">Sélectionnez un territoire</option>
            <option v-for="p in pays" :key="p" :value="p">{{ p }}</option>
          </AfricansChamp>
          <AfricansChamp v-model="form.ville" libelle="Ville" :maxlength="200" placeholder="Ex: Dakar" />
        </div>

        <div class="grid gap-5 md:grid-cols-2">
          <AfricansChamp
            v-model="form.site_web_url"
            libelle="Site web"
            type="url"
            :maxlength="500"
            placeholder="https://exemple.org"
          />
          <AfricansChamp
            v-model="form.lien_reseau_social"
            libelle="Lien réseau social"
            type="url"
            :maxlength="500"
            placeholder="https://facebook.com/…"
          />
        </div>
      </template>

      <!-- ─── Étape 3 : contacts et couverture ─── -->
      <template v-else>
        <div class="flex flex-col gap-4">
          <div>
            <p class="text-[14px]/[1.4] font-bold text-af-encre">Contacts de l'initiateur</p>
            <p class="text-[12px] text-af-atone">Jusqu'à deux contacts (facultatif).</p>
          </div>

          <div class="flex flex-col gap-3 rounded-lg border border-af-bordure p-3">
            <p class="text-[14px]/[1.4] font-bold text-af-corps">Contact 1</p>
            <div class="grid gap-3 md:grid-cols-2">
              <AfricansChamp v-model="form.contact1_courriel" libelle="Courriel" type="email" :maxlength="255" />
              <AfricansChamp v-model="form.contact1_telephone" libelle="Téléphone" type="tel" :maxlength="50" />
            </div>
            <AfricansChamp v-model="form.contact1_adresse" libelle="Adresse géographique" :maxlength="350" />
          </div>

          <div class="flex flex-col gap-3 rounded-lg border border-af-bordure p-3">
            <p class="text-[14px]/[1.4] font-bold text-af-corps">Contact 2</p>
            <div class="grid gap-3 md:grid-cols-2">
              <AfricansChamp v-model="form.contact2_courriel" libelle="Courriel" type="email" :maxlength="255" />
              <AfricansChamp v-model="form.contact2_telephone" libelle="Téléphone" type="tel" :maxlength="50" />
            </div>
            <AfricansChamp v-model="form.contact2_adresse" libelle="Adresse géographique" :maxlength="350" />
          </div>
        </div>

        <div class="flex flex-col gap-3">
          <p class="text-[14px]/[1.4] font-bold text-af-encre">Image de couverture</p>

          <div v-if="form.couverturePreview" class="relative">
            <img :src="form.couverturePreview" alt="" class="h-48 w-full rounded-lg border border-af-bordure object-cover" />
            <button
              type="button"
              class="absolute top-2 right-2 grid size-8 place-items-center rounded-full bg-af-live text-white transition hover:opacity-90"
              aria-label="Retirer l'image"
              @click="removeImage"
            >
              <font-awesome-icon icon="fa-solid fa-xmark" />
            </button>
          </div>

          <div v-else class="rounded-lg border-2 border-dashed border-af-bordure p-6 text-center transition hover:border-af-chocolat">
            <font-awesome-icon icon="fa-solid fa-cloud-arrow-up" class="mb-3 text-3xl text-af-atone-2" />
            <p class="text-[14px]/[1.4] text-af-corps">Cliquez pour ajouter une image</p>
            <p class="mb-3 text-[12px] text-af-atone-2">JPG, PNG, WebP — 5 Mo maximum</p>
            <input
              type="file"
              accept="image/jpeg,image/png,image/webp"
              class="w-full cursor-pointer text-[14px]/[1.4] text-af-corps file:mr-4 file:cursor-pointer file:rounded-lg file:border-0 file:bg-af-chocolat/10 file:px-4 file:py-2 file:text-[14px] file:font-bold file:text-af-chocolat hover:file:bg-af-chocolat/20"
              @change="handleFileChange"
            />
          </div>
        </div>
      </template>
    </form>

    <template #actions>
      <button
        type="button"
        class="mr-auto text-base font-bold text-af-corps transition hover:opacity-70"
        @click="emit('close')"
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
        form="form-initiative"
        :desactive="form.loading || !isFormValid"
        :tourne="form.loading"
        :icone="form.loading ? 'fa-solid fa-spinner' : 'fa-solid fa-paper-plane'"
      >
        {{ form.loading ? 'Publication en cours…' : "Publier l'initiative" }}
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>
