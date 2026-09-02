<script setup lang="ts">
import { useUserStore } from '~/stores/user'
import type { BiblioHumaineAPI, SpecialiteAPI, DemandeBiblioHumaine } from '~/composables/useBibliothequeHumaine'

/**
 * Humantech : porté sur le gabarit de la refonte.
 *
 * Recherche débattue, filtre par spécialité, pagination serveur, bannière de
 * statut de la demande et formulaire d'inscription : tout est conservé.
 *
 * Quatre choses fabriquées disparaissent de la carte :
 *   - `fonction || 'Expert'` et `pays || 'International'` : deux mentions
 *     affichées pour qui n'avait rien renseigné ;
 *   - la biographie de repli « Découvrez mon profil pour en savoir plus… » ;
 *   - l'avatar de repli, qui appelait **ui-avatars.com**, une requête vers un
 *     tiers par personne sans photo. Il devient des initiales.
 * La biographie n'apparaît plus au seul survol : sur un écran tactile, elle
 * était inatteignable. La photo passe par `urlMedia` (le backend renvoie un
 * chemin relatif servi sur SON port).
 *
 * Le formulaire employait `checkbox checkbox-success` et `loading-spinner`,
 * classes **daisyUI**, que la constitution réserve au back-office.
 */
definePageMeta({ layout: false })

useHead({
  title: 'Humantech : Bibliothèques humaines | AfricanS',
  meta: [
    { name: 'description', content: 'Découvrez les personnes-livres et partagez des histoires vivantes.' }],
})

const userStore = useUserStore()
const {
  chargement,
  erreur,
  listerBiblios,
  inscrireBiblioHumaine,
  listerSpecialites,
  obtenirMaDemande,
} = useBibliothequeHumaine()

const maDemande = computed<DemandeBiblioHumaine | null>(
  () => userStore.user ? obtenirMaDemande(userStore.user.id) : null,
)

// ─── État ─────────────────────────────────────────────────────────────────

const biblios = ref<BiblioHumaineAPI[]>([])
const specialitesDisponibles = ref<SpecialiteAPI[]>([])
const filterTypes = ref<string[]>(['Tous'])
const searchQuery = ref('')
const selectedFilter = ref('Tous')
const currentPage = ref(1)
const totalPages = ref(1)

const modaleInscription = ref(false)
const decouverteOuverte = ref(false)
const inscriptionErreur = ref<string | null>(null)
const inscriptionEnCours = ref(false)

const formInscription = reactive({
  fonction: '',
  pays: '',
  biographie: '',
  specialites: [] as string[],
})

const PAYS = [
  'Algérie', 'Angola', 'Bénin', 'Botswana', 'Burkina Faso', 'Burundi',
  'Cameroun', 'Cap-Vert', 'Centrafrique', 'Comores', 'Congo', 'Côte d\'Ivoire',
  'Djibouti', 'Égypte', 'Érythrée', 'Éthiopie', 'Gabon', 'Gambie', 'Ghana',
  'Guinée', 'Guinée-Bissau', 'Guinée Équatoriale', 'Kenya', 'Lesotho',
  'Liberia', 'Libye', 'Madagascar', 'Malawi', 'Mali', 'Maroc', 'Maurice',
  'Mauritanie', 'Mozambique', 'Namibie', 'Niger', 'Nigeria', 'Ouganda',
  'RD Congo', 'Rwanda', 'São Tomé-et-Príncipe', 'Sénégal', 'Seychelles',
  'Sierra Leone', 'Somalie', 'Soudan', 'Soudan du Sud', 'Eswatini',
  'Tanzanie', 'Tchad', 'Togo', 'Tunisie', 'Zambie', 'Zimbabwe', 'Afrique du Sud',
]

const BIO_MIN = 20

const formulaireValide = computed(() =>
  formInscription.fonction.trim().length > 0
  && formInscription.biographie.trim().length >= BIO_MIN
  && formInscription.specialites.length > 0)

// ─── Chargement ───────────────────────────────────────────────────────────

const chargerDonnees = async () => {
  const resultat = await listerBiblios({
    recherche: searchQuery.value || undefined,
    specialite: selectedFilter.value !== 'Tous' ? selectedFilter.value : undefined,
    page: currentPage.value,
    par_page: 12,
  })

  if (resultat) {
    biblios.value = resultat.bibliotheques
    totalPages.value = resultat.total_pages
  }
}

const chargerSpecialites = async () => {
  const specs = await listerSpecialites()
  if (specs) {
    specialitesDisponibles.value = specs
    filterTypes.value = ['Tous', ...specs.map(s => s.nom)]
  }
}

let searchTimeout: ReturnType<typeof setTimeout> | null = null
watch(searchQuery, () => {
  if (searchTimeout) clearTimeout(searchTimeout)
  searchTimeout = setTimeout(() => {
    currentPage.value = 1
    chargerDonnees()
  }, 400)
})

watch(selectedFilter, () => {
  currentPage.value = 1
  chargerDonnees()
})

const changerPage = (page: number) => {
  currentPage.value = page
  chargerDonnees()
}

onMounted(async () => {
  await Promise.all([chargerDonnees(), chargerSpecialites()])
})

// ─── Inscription ──────────────────────────────────────────────────────────

const soumettreInscription = async () => {
  if (!formulaireValide.value) return

  inscriptionEnCours.value = true
  inscriptionErreur.value = null

  const resultat = await inscrireBiblioHumaine({
    specialites: formInscription.specialites,
    biographie: formInscription.biographie.trim(),
    fonction: formInscription.fonction.trim(),
    pays: formInscription.pays || undefined,
  })

  inscriptionEnCours.value = false

  if (resultat) {
    modaleInscription.value = false
    formInscription.fonction = ''
    formInscription.pays = ''
    formInscription.biographie = ''
    formInscription.specialites = []
    chargerDonnees()
  }
  else {
    inscriptionErreur.value = erreur.value || 'Erreur lors de l\'inscription.'
  }
}

// ─── Affichage ────────────────────────────────────────────────────────────

const initiales = (b: BiblioHumaineAPI) =>
  `${b.prenom?.[0] ?? ''}${b.nom?.[0] ?? ''}`.toUpperCase()

const dateFr = (d: string) => new Date(d).toLocaleDateString('fr-FR')
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Humantech"
        sous-titre="Rencontrer celles et ceux qui portent la mémoire du continent"
        image="/images/biblio.png"
        aide="C'est quoi Humantech ?"
        @aide="decouverteOuverte = true"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Humantech' }]">
        <template #action>
          <AfricansBouton icone="fa-solid fa-user-plus" @click="modaleInscription = true">
            Devenir bibliothèque humaine
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-6">
      <!-- Statut de la demande du membre connecté -->
      <template v-if="maDemande">
        <div
          v-if="maDemande.statut === 'en_attente'"
          class="flex items-start gap-3 rounded-[10px] border border-af-bordure bg-af-fond p-4"
        >
          <font-awesome-icon icon="fa-solid fa-clock" class="mt-0.5 shrink-0 text-af-chocolat" />
          <div>
            <p class="text-[14px]/[1.4] font-bold">Votre demande est en cours d'examen</p>
            <p class="mt-1 text-[12px]/[1.4] text-af-corps">
              Soumise le {{ dateFr(maDemande.dateSubmission) }}, un administrateur la traitera prochainement.
            </p>
          </div>
        </div>

        <div
          v-else-if="maDemande.statut === 'valide'"
          class="flex items-start gap-3 rounded-[10px] border border-af-vert/30 bg-af-vert/5 p-4"
        >
          <font-awesome-icon icon="fa-solid fa-circle-check" class="mt-0.5 shrink-0 text-af-vert" />
          <div>
            <p class="text-[14px]/[1.4] font-bold">Vous êtes une bibliothèque humaine</p>
            <p class="mt-1 text-[12px]/[1.4] text-af-corps">
              Votre inscription a été validée. Votre profil est visible dans la liste ci-dessous.
            </p>
          </div>
        </div>

        <div
          v-else-if="maDemande.statut === 'rejete'"
          class="flex items-start gap-3 rounded-[10px] border border-af-live/30 bg-af-live/5 p-4"
        >
          <font-awesome-icon icon="fa-solid fa-circle-xmark" class="mt-0.5 shrink-0 text-af-live" />
          <div>
            <p class="text-[14px]/[1.4] font-bold">Votre demande n'a pas été retenue</p>
            <p v-if="maDemande.commentaireAdmin" class="mt-1 text-[12px]/[1.4] text-af-corps italic">
              « {{ maDemande.commentaireAdmin }} »
            </p>
            <p class="mt-1 text-[12px]/[1.4] text-af-corps">
              Vous pouvez soumettre une nouvelle candidature.
            </p>
          </div>
        </div>
      </template>

      <label class="relative block">
        <span class="sr-only">Rechercher une personne ressource</span>
        <font-awesome-icon
          icon="fa-solid fa-magnifying-glass"
          class="pointer-events-none absolute top-1/2 left-4 -translate-y-1/2 text-af-atone-2"
        />
        <input
          v-model="searchQuery"
          type="search"
          placeholder="Nom, spécialité, mot-clé…"
          class="h-11 w-full rounded-[10px] border border-af-bordure bg-white pr-4 pl-11 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
        />
      </label>

      <div class="flex flex-wrap gap-2">
        <button
          v-for="type in filterTypes"
          :key="type"
          type="button"
          class="rounded-full px-3 py-1.5 text-[12px]/[1.4] font-bold transition"
          :class="selectedFilter === type ? 'bg-af-chocolat text-white' : 'bg-af-fond text-af-corps hover:bg-af-bordure'"
          :aria-pressed="selectedFilter === type"
          @click="selectedFilter = type"
        >
          {{ type }}
        </button>
      </div>

      <div v-if="chargement" class="grid gap-5 sm:grid-cols-2">
        <div v-for="n in 4" :key="n" class="h-64 animate-pulse rounded-[10px] bg-af-bordure" />
      </div>

      <p v-else-if="erreur" role="alert" class="flex items-center gap-2 rounded-[10px] bg-af-live/10 p-3 text-[14px]/[1.4] text-af-live">
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" />
        {{ erreur }}
      </p>

      <template v-else-if="biblios.length">
        <div class="grid gap-5 sm:grid-cols-2">
          <NuxtLink
            v-for="biblio in biblios"
            :key="biblio.id"
            :to="`/profil/${biblio.userId}`"
            class="group flex flex-col gap-3 rounded-[10px] border border-af-bordure bg-white p-5 transition hover:border-af-chocolat"
          >
            <div class="flex items-start gap-4">
              <img
                v-if="urlMedia(biblio.photoUrl)"
                :src="urlMedia(biblio.photoUrl)!"
                :alt="`${biblio.prenom} ${biblio.nom}`"
                class="size-16 shrink-0 rounded-full object-cover"
              />
              <span
                v-else
                class="grid size-16 shrink-0 place-items-center rounded-full bg-af-chocolat/15 text-[17px]/[1.4] font-bold text-af-chocolat"
              >{{ initiales(biblio) }}</span>

              <div class="flex min-w-0 flex-1 flex-col gap-1">
                <p class="truncate text-[17px]/[1.4] font-bold text-af-encre transition group-hover:text-af-chocolat">
                  {{ biblio.prenom }} <span class="uppercase">{{ biblio.nom }}</span>
                </p>
                <p v-if="biblio.fonction" class="flex items-center gap-1.5 text-[12px]/[1.4] text-af-corps">
                  <font-awesome-icon icon="fa-solid fa-briefcase" />
                  {{ biblio.fonction }}
                </p>
                <p v-if="biblio.pays" class="flex items-center gap-1.5 text-[12px]/[1.4] text-af-atone">
                  <font-awesome-icon icon="fa-solid fa-location-dot" />
                  {{ biblio.pays }}
                </p>
              </div>
            </div>

            <AfricansEtiquette v-if="biblio.specialite" ton="vert" class="self-start">
              {{ biblio.specialite }}
            </AfricansEtiquette>

            <p v-if="biblio.biographie" class="line-clamp-3 text-[12px]/[1.4] text-af-corps italic">
              « {{ biblio.biographie }} »
            </p>

            <span class="mt-auto flex items-center gap-2 pt-1 text-[14px]/[1.4] font-bold text-af-chocolat">
              Voir le profil
              <font-awesome-icon icon="fa-solid fa-arrow-right" class="transition group-hover:translate-x-1" />
            </span>
          </NuxtLink>
        </div>

        <nav v-if="totalPages > 1" class="flex flex-wrap items-center justify-center gap-2">
          <button
            v-for="p in totalPages"
            :key="p"
            type="button"
            class="size-10 rounded-[10px] text-[14px]/[1.4] font-bold transition"
            :class="p === currentPage ? 'bg-af-chocolat text-white' : 'border border-af-bordure bg-white hover:border-af-chocolat'"
            :aria-current="p === currentPage ? 'page' : undefined"
            @click="changerPage(p)"
          >
            {{ p }}
          </button>
        </nav>
      </template>

      <div v-else class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-chalkboard-user" class="text-4xl text-af-atone-2" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">Aucune bibliothèque humaine trouvée</p>
        <p class="mt-2 text-[14px]/[1.4] text-af-corps">
          Essayez un autre mot-clé ou une autre spécialité.
        </p>
      </div>
    </div>

    <template #rail>
      <AfricansPanneau titre="Transmettre" icone="fa-solid fa-chalkboard-user">
        <div class="flex flex-col gap-3">
          <p class="text-[14px]/[1.4] text-af-corps">
            Aîné, griot, expert, témoin de l'histoire : présentez ce que vous souhaitez transmettre
            et rejoignez le répertoire des personnes ressources.
          </p>
          <AfricansBouton icone="fa-solid fa-user-plus" @click="modaleInscription = true">
            Devenir bibliothèque humaine
          </AfricansBouton>
        </div>
      </AfricansPanneau>
    </template>

    <!-- Inscription -->
    <AfricansModale
      v-model="modaleInscription"
      titre="Devenir bibliothèque humaine"
      sous-titre="Votre candidature sera examinée par un administrateur."
    >
      <!-- Un visiteur ne peut pas candidater : on le lui dit, sans lui montrer
           un formulaire qu'il ne pourrait pas soumettre. -->
      <div v-if="!userStore.user" class="flex flex-col items-center gap-4 py-6 text-center">
        <font-awesome-icon icon="fa-solid fa-right-to-bracket" class="text-3xl text-af-chocolat" />
        <p class="text-[14px]/[1.4] text-af-corps">
          Connectez-vous pour proposer votre candidature.
        </p>
        <AfricansBouton vers="/login?redirect=/bibliotheque/humaine">Se connecter</AfricansBouton>
      </div>

      <form v-else class="flex flex-col gap-5" @submit.prevent="soumettreInscription">
        <AfricansChamp
          v-model="formInscription.fonction"
          libelle="Fonction / Métier"
          placeholder="Ex. : griot et conteur, anthropologue, professeur…"
          obligatoire
        />

        <label class="flex flex-col gap-2">
          <span class="text-[14px]/[1.4] font-bold">Territoire d'origine</span>
          <select
            v-model="formInscription.pays"
            class="h-11 w-full rounded-[10px] border border-af-bordure bg-white px-3 text-[14px]/[1.4] focus:outline-2 focus:outline-af-chocolat"
          >
            <option value="">Sélectionner un territoire</option>
            <option v-for="p in PAYS" :key="p" :value="p">{{ p }}</option>
          </select>
        </label>

        <AfricansChamp
          v-model="formInscription.biographie"
          libelle="Biographie"
          type="textarea"
          placeholder="Décrivez votre parcours, vos connaissances et ce que vous souhaitez partager…"
          :aide="`${formInscription.biographie.length} / ${BIO_MIN} caractères minimum`"
          obligatoire
        />

        <div class="flex flex-col gap-2">
          <p class="text-[14px]/[1.4] font-bold">Spécialités <span class="text-af-live">*</span></p>
          <div class="flex max-h-48 flex-col overflow-y-auto rounded-[10px] border border-af-bordure p-2">
            <label
              v-for="spec in specialitesDisponibles"
              :key="spec.id"
              class="flex cursor-pointer items-center gap-3 rounded-lg px-2 py-1.5 text-[14px]/[1.4] transition hover:bg-af-fond"
            >
              <input
                v-model="formInscription.specialites"
                type="checkbox"
                :value="spec.nom"
                class="size-4 accent-af-chocolat"
              />
              {{ spec.nom }}
            </label>
          </div>
        </div>

        <p v-if="inscriptionErreur" role="alert" class="flex items-start gap-2 text-[12px]/[1.4] text-af-live">
          <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="mt-0.5" />
          {{ inscriptionErreur }}
        </p>
      </form>

      <template v-if="userStore.user" #actions>
        <button
          type="button"
          class="text-base font-bold text-af-corps transition hover:opacity-70"
          @click="modaleInscription = false"
        >
          Annuler
        </button>
        <AfricansBouton
          :desactive="!formulaireValide || inscriptionEnCours"
          :tourne="inscriptionEnCours"
          :icone="inscriptionEnCours ? 'fa-solid fa-spinner' : undefined"
          @click="soumettreInscription"
        >
          {{ inscriptionEnCours ? 'Inscription en cours…' : 'Valider mon inscription' }}
        </AfricansBouton>
      </template>
    </AfricansModale>

    <BibliothequeDecouverteHumantech v-model="decouverteOuverte" />
  </NuxtLayout>
</template>
