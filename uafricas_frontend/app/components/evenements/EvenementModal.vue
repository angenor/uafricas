<template>
  <AfricansModale
    :model-value="show"
    titre="Proposer un événement"
    :sous-titre="`Étape ${etape + 1} sur ${ETAPES.length} : ${ETAPES[etape]!.titre}`"
    icone="fa-solid fa-calendar-day"
    @update:model-value="!$event && emit('close')"
  >
    <div class="flex flex-col gap-5">
      <!-- Fil des étapes -->
      <nav class="flex gap-2" aria-label="Étapes de la proposition">
        <button
          v-for="(e, i) in ETAPES"
          :key="e.titre"
          type="button"
          class="flex min-w-0 flex-1 items-center gap-2 rounded-lg border px-3 py-2 text-left transition"
          :class="i === etape ? 'border-af-chocolat bg-af-chocolat/[0.07]' : 'border-af-bordure hover:border-af-chocolat'"
          :aria-current="i === etape ? 'step' : undefined"
          @click="allerA(i)"
        >
          <span
            class="grid size-6 shrink-0 place-items-center rounded-full text-[11px] font-bold"
            :class="i < etape ? 'bg-af-vert text-white' : i === etape ? 'bg-af-chocolat text-white' : 'bg-af-fond text-af-atone'"
          >
            <font-awesome-icon v-if="i < etape" icon="fa-solid fa-check" />
            <template v-else>{{ i + 1 }}</template>
          </span>
          <span class="min-w-0 truncate text-[12px]/[1.3] font-bold" :class="i === etape ? 'text-af-chocolat' : 'text-af-corps'">
            {{ e.titre }}
          </span>
        </button>
      </nav>

      <!-- ─── 1. L'événement ───
           `v-show` et non `v-if` : l'éditeur de description est monté une
           seule fois. Le démonter à chaque changement d'étape le forcerait à
           se réinitialiser, et son contenu ne survit pas toujours à un
           remontage. -->
      <div v-show="etape === 0" class="flex flex-col gap-5">
        <AfricansChamp v-model="form.titre" libelle="Titre *" placeholder="Titre de l'événement" />

        <div class="flex flex-col gap-2">
          <span class="text-[14px]/[1.4] text-af-atone italic">Description *</span>
          <CommonEditorJs
            v-if="show"
            id="evenement-description-editor"
            ref="editorRef"
            v-model="form.descriptionData"
            placeholder="Décrivez votre événement…"
            :tools="['header', 'list', 'paragraph', 'quote', 'delimiter', 'marker', 'underline']"
            min-height="200px"
          />
        </div>

        <AfricansChamp v-model="form.thematique" libelle="Thématique *" type="select">
          <option value="">Choisir une thématique</option>
          <option v-for="t in THEMATIQUES_EVENEMENT" :key="t" :value="t">{{ t }}</option>
        </AfricansChamp>

        <!-- L'astérisque de ce libellé était MENSONGER : `isFormValid` n'a
             jamais exigé de couverture. Le champ reste facultatif, il le dit. -->
        <label class="flex cursor-pointer flex-col items-center gap-2 rounded-[10px] border-2 border-dashed border-af-bordure p-4 text-center transition hover:border-af-chocolat">
          <font-awesome-icon icon="fa-solid fa-image" class="text-2xl text-af-chocolat" />
          <span class="text-[14px]/[1.4] font-bold">Image de couverture</span>
          <span class="w-full truncate text-[12px]/[1.4] text-af-atone">
            {{ form.couverture_file ? form.couverture_file.name : 'Facultative' }}
          </span>
          <input type="file" class="sr-only" accept="image/*" @change="handleFileChange" />
        </label>
      </div>

      <!-- ─── 2. Quand et où ─── -->
      <div v-show="etape === 1" class="flex flex-col gap-5">
        <div class="grid gap-4 sm:grid-cols-2">
          <AfricansChamp v-model="form.type" libelle="Type *" type="select">
            <option value="">Choisir un type</option>
            <option value="En ligne">En ligne</option>
            <option value="En présentiel">En présentiel</option>
            <option value="Hybride">Hybride</option>
          </AfricansChamp>

          <AfricansChamp v-model="form.pays" libelle="Territoire *" type="select">
            <option value="">Choisir un territoire</option>
            <option v-for="pays in PAYS_AFRICAINS" :key="pays" :value="pays">{{ pays }}</option>
          </AfricansChamp>
        </div>

        <AfricansChamp v-model="form.ville" libelle="Ville *" placeholder="Ville" />

        <div class="grid gap-4 sm:grid-cols-2">
          <label class="flex flex-col gap-2">
            <span class="text-[14px]/[1.4] text-af-atone italic">Début *</span>
            <input
              v-model="form.date_heure_debut"
              type="datetime-local"
              class="h-11 rounded-md border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:border-af-chocolat focus:outline-none"
            />
          </label>
          <label class="flex flex-col gap-2">
            <span class="text-[14px]/[1.4] text-af-atone italic">Fin *</span>
            <input
              v-model="form.date_heure_fin"
              type="datetime-local"
              class="h-11 rounded-md border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-none"
              :class="dateFinValide ? 'border-af-bordure focus:border-af-chocolat' : 'border-af-live'"
            />
          </label>
        </div>

        <!-- L'adresse précise concerne les événements physiques ; le lien,
             ceux en ligne. Un hybride veut les deux. -->
        <AfricansChamp
          v-if="afficheAdresse"
          v-model="form.adresse"
          libelle="Adresse précise"
          placeholder="Rue, quartier, bâtiment…"
        />
        <AfricansChamp
          v-if="afficheLien"
          v-model="form.lien_en_ligne"
          libelle="Lien de connexion"
          type="url"
          placeholder="https://…"
        />

        <label class="flex flex-col gap-2">
          <span class="text-[14px]/[1.4] text-af-atone italic">Nombre de places</span>
          <input
            v-model.number="form.nombre_places"
            type="number"
            min="1"
            placeholder="Laisser vide si illimité"
            class="h-11 rounded-md border border-af-bordure bg-white px-3 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:border-af-chocolat focus:outline-none"
          />
        </label>
      </div>

      <!-- ─── 3. L'organisateur ─── -->
      <div v-show="etape === 2" class="flex flex-col gap-5">
        <div class="flex flex-col gap-2">
          <span class="text-[14px]/[1.4] text-af-atone italic">Vous publiez *</span>
          <div class="grid gap-3 sm:grid-cols-2">
            <label
              v-for="choix in ORGANISATEURS"
              :key="choix.valeur"
              class="flex cursor-pointer items-start gap-3 rounded-[10px] border p-4 transition"
              :class="form.type_organisateur === choix.valeur
                ? 'border-af-chocolat bg-af-chocolat/[0.07]'
                : 'border-af-bordure hover:border-af-chocolat'"
            >
              <input v-model="form.type_organisateur" type="radio" :value="choix.valeur" class="mt-1 accent-af-chocolat" />
              <span class="min-w-0">
                <span class="block text-[14px]/[1.4] font-bold text-af-encre">{{ choix.libelle }}</span>
                <span class="block text-[12px]/[1.4] text-af-atone">{{ choix.aide }}</span>
              </span>
            </label>
          </div>
        </div>

        <AfricansChamp
          v-if="form.type_organisateur === 'organisation'"
          v-model="form.contact_nom"
          libelle="Nom de l'organisation *"
          placeholder="Nom de la structure organisatrice"
        />

        <div class="grid gap-4 sm:grid-cols-2">
          <AfricansChamp v-model="form.contact_email" libelle="Courriel de contact" type="email" placeholder="contact@exemple.org" />
          <AfricansChamp v-model="form.contact_telephone" libelle="Téléphone" type="tel" placeholder="+225 07 00 00 00 00" />
        </div>

        <AfricansChamp v-model="form.contact_site_web" libelle="Site web" type="url" placeholder="https://www.exemple.org" />
      </div>

      <p v-if="messageEtape" role="alert" class="flex items-start gap-2 text-[12px]/[1.4] text-af-live">
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="mt-0.5 shrink-0" />
        {{ messageEtape }}
      </p>
    </div>

    <template #actions>
      <button
        v-if="etape === 0"
        type="button"
        class="text-base font-bold text-af-corps transition hover:opacity-70"
        @click="emit('close')"
      >
        Annuler
      </button>
      <AfricansBouton v-else variante="secondaire" icone="fa-solid fa-arrow-left" @click="precedent">
        Précédent
      </AfricansBouton>

      <AfricansBouton v-if="etape < ETAPES.length - 1" icone="fa-solid fa-arrow-right" @click="suivant">
        Suivant
      </AfricansBouton>
      <AfricansBouton v-else icone="fa-solid fa-paper-plane" @click="handleSubmit">
        Soumettre
      </AfricansBouton>
    </template>
  </AfricansModale>
</template>

<script setup lang="ts">
import { PAYS_AFRICAINS, THEMATIQUES_EVENEMENT } from '~/composables/useEvenements'
import { editorJsToHtml, type EditorJsData } from '~/composables/useEditorJs'

defineProps<{
  show: boolean
}>()

const emit = defineEmits<{
  close: []
  submit: [data: {
    titre: string
    description: string
    type: string
    thematique: string
    pays: string
    ville: string
    date_heure_debut: string
    date_heure_fin: string
    adresse: string
    lien_en_ligne: string
    nombre_places: number | null
    type_organisateur: 'personnel' | 'organisation'
    contact_nom: string
    contact_email: string
    contact_telephone: string
    contact_site_web: string
    couverture_file: File | null
  }]
}>()

const editorRef = ref<{ save: () => Promise<EditorJsData | null>; clear: () => Promise<void> } | null>(null)

const form = reactive({
  titre: '',
  descriptionData: undefined as EditorJsData | undefined,
  type: '',
  thematique: '',
  pays: '',
  ville: '',
  date_heure_debut: '',
  date_heure_fin: '',
  adresse: '',
  lien_en_ligne: '',
  nombre_places: null as number | null,
  type_organisateur: 'personnel' as 'personnel' | 'organisation',
  contact_nom: '',
  contact_email: '',
  contact_telephone: '',
  contact_site_web: '',
  couverture_file: null as File | null
})

// L'adresse précise concerne les événements physiques ; le lien, ceux en ligne.
const afficheAdresse = computed(() => form.type === 'En présentiel' || form.type === 'Hybride')
const afficheLien = computed(() => form.type === 'En ligne' || form.type === 'Hybride')

const hasDescription = computed(() => {
  return form.descriptionData && form.descriptionData.blocks && form.descriptionData.blocks.length > 0
})

// Au nom d'une organisation : le nom de l'organisation est requis.
const contactValide = computed(() =>
  form.type_organisateur !== 'organisation' || form.contact_nom.trim().length > 0
)

// Constat #16 : la fin doit être postérieure au début (fini les créneaux 13:04-13:04)
const dateFinValide = computed(() => {
  if (!form.date_heure_debut || !form.date_heure_fin) return true
  return new Date(form.date_heure_fin) > new Date(form.date_heure_debut)
})

const ETAPES = [
  { titre: 'L\'événement', icone: 'fa-solid fa-calendar-day' },
  { titre: 'Quand et où', icone: 'fa-solid fa-location-dot' },
  { titre: 'L\'organisateur', icone: 'fa-solid fa-user-tie' },
] as const

const ORGANISATEURS = [
  { valeur: 'personnel' as const, libelle: 'En mon nom propre', aide: "Vous êtes l'organisateur·rice" },
  { valeur: 'organisation' as const, libelle: "Au nom d'une organisation", aide: 'Le nom de la structure sera affiché' },
]

const etape = ref(0)
const messageEtape = ref<string | null>(null)

/**
 * Ce qui manque à une étape, ÉNONCÉ. La validation d'ensemble existait déjà
 * (`isFormValid`) mais se contentait de griser le bouton d'envoi : on voyait
 * qu'on ne pouvait pas soumettre, jamais pourquoi, et les dix-sept champs
 * étaient sur un seul écran.
 */
const manqueEtape = (i: number): string | null => {
  if (i === 0) {
    if (!form.titre.trim()) return 'Le titre est nécessaire.'
    if (!hasDescription.value) return 'La description est nécessaire.'
    if (!form.thematique) return 'Choisissez une thématique.'
  }
  if (i === 1) {
    if (!form.type) return 'Choisissez un type d\'événement.'
    if (!form.pays) return 'Choisissez un territoire.'
    if (!form.ville.trim()) return 'La ville est nécessaire.'
    if (!form.date_heure_debut) return 'La date de début est nécessaire.'
    if (!form.date_heure_fin) return 'La date de fin est nécessaire.'
    if (!dateFinValide.value) return 'La fin doit être postérieure au début.'
  }
  if (i === 2) {
    if (!contactValide.value) return 'Le nom de l\'organisation est nécessaire.'
  }
  return null
}

const suivant = () => {
  const manque = manqueEtape(etape.value)
  if (manque) { messageEtape.value = manque; return }
  messageEtape.value = null
  etape.value += 1
}

const precedent = () => {
  messageEtape.value = null
  etape.value -= 1
}

const allerA = (i: number) => {
  // On ne saute en avant que par des étapes déjà complètes.
  if (i > etape.value) {
    for (let k = etape.value; k < i; k++) {
      const manque = manqueEtape(k)
      if (manque) { etape.value = k; messageEtape.value = manque; return }
    }
  }
  messageEtape.value = null
  etape.value = i
}

const isFormValid = computed(() => {
  return form.titre &&
    hasDescription.value &&
    form.type &&
    form.thematique &&
    form.pays &&
    form.ville &&
    form.date_heure_debut &&
    form.date_heure_fin &&
    dateFinValide.value &&
    contactValide.value
})

const handleFileChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  if (target.files && target.files[0]) {
    form.couverture_file = target.files[0]
  }
}

const handleSubmit = async () => {
  // Toutes les étapes sont revérifiées : atteindre la dernière ne doit pas
  // suffire à envoyer un dossier incomplet, et la modale ramène sur l'étape
  // fautive au lieu de simplement griser le bouton.
  for (let i = 0; i < ETAPES.length; i++) {
    const manque = manqueEtape(i)
    if (manque) { etape.value = i; messageEtape.value = manque; return }
  }
  if (!isFormValid.value) return
  messageEtape.value = null

  // Sauvegarder l'éditeur pour obtenir les données finales
  let descriptionHtml = ''
  if (editorRef.value) {
    const savedData = await editorRef.value.save()
    if (savedData) {
      descriptionHtml = editorJsToHtml(savedData)
    }
  } else if (form.descriptionData) {
    descriptionHtml = editorJsToHtml(form.descriptionData)
  }

  emit('submit', {
    titre: form.titre,
    description: descriptionHtml,
    type: form.type,
    thematique: form.thematique,
    pays: form.pays,
    ville: form.ville,
    date_heure_debut: form.date_heure_debut,
    date_heure_fin: form.date_heure_fin,
    adresse: afficheAdresse.value ? form.adresse : '',
    lien_en_ligne: afficheLien.value ? form.lien_en_ligne : '',
    nombre_places: form.nombre_places,
    type_organisateur: form.type_organisateur,
    contact_nom: form.type_organisateur === 'organisation' ? form.contact_nom : '',
    contact_email: form.contact_email,
    contact_telephone: form.contact_telephone,
    contact_site_web: form.contact_site_web,
    couverture_file: form.couverture_file
  })

  // Reset form
  form.titre = ''
  form.descriptionData = undefined
  form.type = ''
  form.thematique = ''
  form.pays = ''
  form.ville = ''
  form.date_heure_debut = ''
  form.date_heure_fin = ''
  form.adresse = ''
  form.lien_en_ligne = ''
  form.nombre_places = null
  form.type_organisateur = 'personnel'
  form.contact_nom = ''
  form.contact_email = ''
  form.contact_telephone = ''
  form.contact_site_web = ''
  form.couverture_file = null
  etape.value = 0
  messageEtape.value = null
  if (editorRef.value) {
    await editorRef.value.clear()
  }
}
</script>
