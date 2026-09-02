<template>
  <!-- Ni carte ni en-tête : la coque AfricansModale porte déjà le cadre, le
       titre et le sous-titre. Les redoubler donnait deux fois « Proposer une
       salle » à l'écran, l'un dans l'autre. -->
  <form id="form-proposition-salle" class="flex flex-col gap-5" @submit.prevent="soumettre">
    <p
      v-if="messageSucces"
      class="flex items-start gap-2 rounded-lg border border-af-vert/20 bg-af-vert/5 px-4 py-3 text-[14px]/[1.4] text-af-vert"
    >
      <font-awesome-icon icon="fa-solid fa-circle-check" class="mt-0.5 shrink-0" />
      {{ messageSucces }}
    </p>

    <p
      v-if="messageErreur"
      class="flex items-start gap-2 rounded-lg border border-af-live/20 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live"
    >
      <font-awesome-icon icon="fa-solid fa-triangle-exclamation" class="mt-0.5 shrink-0" />
      {{ messageErreur }}
    </p>

    <AfricansChamp
      v-model="form.titre"
      libelle="Titre"
      :maxlength="350"
      placeholder="Ex. Apprenons le wolof ensemble"
      obligatoire
    />

    <div class="grid gap-5 md:grid-cols-2">
      <AfricansChamp
        v-model="form.langue_cible"
        libelle="Langue cible"
        :maxlength="100"
        placeholder="Ex. Wolof"
        obligatoire
      />
      <AfricansChamp
        v-model="form.langue_code"
        libelle="Code de langue"
        :maxlength="40"
        placeholder="Ex. wo"
        aide="Facultatif"
      />
    </div>

    <div class="flex flex-col gap-3">
      <AfricansChamp
        v-model="groupeSelection"
        libelle="Groupe ethnique"
        type="select"
        aide="Liste non exhaustive : si votre groupe n'y figure pas, choisissez « Autre » et indiquez son nom."
        obligatoire
      >
        <option value="" disabled>Sélectionner un groupe ethnique</option>
        <option v-for="g in groupesDisponibles" :key="g.id" :value="g.id">
          {{ g.nom }}<span v-if="g.pays_nom"> · {{ g.pays_nom }}</span>
        </option>
        <option :value="AUTRE">Autre (préciser)…</option>
      </AfricansChamp>

      <!-- Champ libre quand « Autre » est retenu -->
      <AfricansChamp
        v-if="groupeSelection === AUTRE"
        v-model="form.groupe_ethnique_libre"
        libelle="Nom du groupe ethnique"
        :maxlength="250"
        placeholder="Ex. Bassa, Sérère, Créole haïtien…"
        obligatoire
      />
    </div>

    <!-- Sélecteur de territoires : liste à cocher groupée par continent. Ce
         n'est pas un AfricansChamp — aucun de ses types ne rend une sélection
         multiple, et le champ de recherche filtre la liste, il ne la saisit
         pas. -->
    <div class="flex flex-col gap-2">
      <p class="text-[14px]/[1.4] text-af-atone italic">
        Territoire d'origine <span class="not-italic text-af-live">*</span>
      </p>

      <input
        v-model="rechercheTerritoire"
        type="text"
        class="h-11 w-full rounded-md border border-af-bordure bg-white px-4 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:border-af-chocolat focus:outline-none"
        placeholder="Rechercher un territoire…"
      >

      <div class="flex max-h-64 flex-col gap-4 overflow-y-auto rounded-md border border-af-bordure bg-af-fond p-3">
        <p v-if="!territoires.length" class="text-[14px]/[1.4] text-af-atone">
          Chargement des territoires…
        </p>
        <p v-else-if="!territoiresParContinent.length" class="text-[14px]/[1.4] text-af-atone">
          Aucun territoire ne correspond à votre recherche.
        </p>
        <fieldset v-for="bloc in territoiresParContinent" v-else :key="bloc.continent">
          <legend class="mb-2 text-[12px] font-bold text-af-atone uppercase">
            {{ bloc.continent }}
          </legend>
          <div class="grid grid-cols-2 gap-2 sm:grid-cols-3">
            <label
              v-for="p in bloc.territoires"
              :key="p.id"
              class="flex cursor-pointer items-center gap-2 text-[14px]/[1.4] text-af-corps transition hover:text-af-encre"
            >
              <input
                v-model="form.pays_origine_ids"
                type="checkbox"
                :value="p.id"
                class="size-4 rounded border-af-bordure accent-af-chocolat"
              >
              <span>{{ p.nom }}</span>
            </label>
          </div>
        </fieldset>
      </div>

      <p class="text-[12px]/[1.4] text-af-atone">
        Sélectionnez au moins un territoire où la langue cible est parlée, y compris
        hors d'Afrique (diaspora, créoles, langues afro-descendantes).
        {{ form.pays_origine_ids.length }} sélectionné(s).
      </p>
    </div>

    <AfricansChamp
      v-model="form.description"
      libelle="Description"
      type="textarea"
      :lignes="3"
      placeholder="Présentez brièvement la salle envisagée…"
      obligatoire
    />

    <AfricansChamp
      v-model="form.justification"
      libelle="Justification"
      type="textarea"
      :lignes="3"
      placeholder="Pourquoi cette salle serait utile ? Quel public ?"
      obligatoire
    />

    <p class="text-[12px]/[1.4] text-af-atone">
      Votre proposition sera examinée par un administrateur de la plateforme.
    </p>
  </form>
</template>

<script setup lang="ts">
import { computed, reactive, ref } from 'vue'
import type {
  GroupeEthniqueResume,
  PropositionSalle,
  SoumettrePropositionPayload,
  TerritoireAPI,
} from '~/composables/useAfrolang'

interface GroupeOption {
  id: string
  nom: string
  pays_nom: string | null
}

const props = defineProps<{
  groupesDisponibles: GroupeOption[]
  territoires: TerritoireAPI[]
}>()

const emit = defineEmits<{
  (e: 'soumis', proposition: PropositionSalle): void
}>()

const { proposerSalle } = useAfrolang()

/** Valeur sentinelle du sélecteur de groupe pour l'option « Autre ». */
const AUTRE = '__autre__'

const enCours = ref(false)
const messageErreur = ref<string | null>(null)
const messageSucces = ref<string | null>(null)

/** Sélection du groupe ethnique : '' (aucun), un UUID de groupe, ou AUTRE. */
const groupeSelection = ref<string>('')

const form = reactive({
  titre: '',
  description: '',
  justification: '',
  langue_cible: '',
  langue_code: '',
  groupe_ethnique_libre: '',
  pays_origine_ids: [] as string[],
})

// ── Territoires groupés par continent (Afrique en tête, ordre fourni par l'API) ──
const rechercheTerritoire = ref('')

const territoiresParContinent = computed(() => {
  const filtre = rechercheTerritoire.value.trim().toLowerCase()
  const groupes = new Map<string, TerritoireAPI[]>()
  for (const t of props.territoires) {
    if (filtre && !t.nom.toLowerCase().includes(filtre)) continue
    const cle = t.continent || 'Autres'
    if (!groupes.has(cle)) groupes.set(cle, [])
    groupes.get(cle)!.push(t)
  }
  return Array.from(groupes, ([continent, territoires]) => ({ continent, territoires }))
})

const groupeValide = computed(() =>
  groupeSelection.value === AUTRE
    ? form.groupe_ethnique_libre.trim().length > 0
    : groupeSelection.value.length > 0,
)

const formulaireValide = computed(() =>
  form.titre.trim().length > 0
  && form.description.trim().length > 0
  && form.justification.trim().length > 0
  && form.langue_cible.trim().length > 0
  && groupeValide.value
  && form.pays_origine_ids.length > 0,
)

const reinitialiser = () => {
  form.titre = ''
  form.description = ''
  form.justification = ''
  form.langue_cible = ''
  form.langue_code = ''
  form.groupe_ethnique_libre = ''
  form.pays_origine_ids = []
  groupeSelection.value = ''
  rechercheTerritoire.value = ''
}

defineExpose({ enCours, formulaireValide })

const soumettre = async () => {
  messageErreur.value = null
  messageSucces.value = null
  if (!formulaireValide.value) return
  enCours.value = true
  try {
    const estAutre = groupeSelection.value === AUTRE
    const payload: SoumettrePropositionPayload = {
      titre: form.titre.trim(),
      description: form.description.trim(),
      justification: form.justification.trim(),
      langue_cible: form.langue_cible.trim(),
      langue_code: form.langue_code?.trim() || null,
      groupe_ethnique_id: estAutre ? null : groupeSelection.value,
      groupe_ethnique_libre: estAutre ? form.groupe_ethnique_libre.trim() : null,
      pays_origine_ids: [...form.pays_origine_ids],
    }
    const proposition = await proposerSalle(payload)
    if (proposition) {
      messageSucces.value = 'Proposition soumise avec succès. Un administrateur vous répondra prochainement.'
      emit('soumis', proposition)
      reinitialiser()
    }
    else {
      messageErreur.value = 'La soumission a échoué. Vérifiez les champs et réessayez.'
    }
  }
  finally {
    enCours.value = false
  }
}

// Évite "unused" lint sur GroupeEthniqueResume si Vue compile en strict
void ({} as GroupeEthniqueResume)
</script>
