<script setup lang="ts">
import type { PersonneListe, CreerPersonneForm } from '~/mocks/arbre-genealogique'
import { useArbreGenealogique } from '~/composables/useArbreGenealogique'
import { useDecouvertes } from '~/composables/useDecouvertes'
import PersonneForm from '~/components/arbre-genealogique/PersonneForm.vue'
import PersonneCard from '~/components/arbre-genealogique/PersonneCard.vue'
import AssistantAjoutPersonne from '~/components/arbre-genealogique/AssistantAjoutPersonne.vue'

/**
 * Rootstree : porté sur le gabarit de la refonte.
 *
 * La logique ne bouge pas : mêmes endpoints, même recherche débattue à 350 ms,
 * même pagination serveur, mêmes deux modes de création (assistant et
 * formulaire). Ce qui change est l'enveloppe : le hero à dégradé devient un
 * bandeau de module, les deux boutons « Découvertes » et « Voir mon arbre »
 * quittent l'en-tête pour le rail, et la modale de présentation passe sur la
 * modale à étapes commune aux autres modules.
 *
 * Le bandeau n'a PAS d'image : aucune illustration de généalogie n'existe dans
 * `public/images/`, et le Figma n'en fournit pas pour ce module. Le dégradé de
 * marque est le repli prévu : en coller une sans rapport serait pire.
 */
definePageMeta({ middleware: 'auth', layout: false })

useHead({ title: 'Rootstree : Mon arbre généalogique | AfricanS' })

const { listerPersonnes, creerPersonne } = useArbreGenealogique()
const { listerDecouvertes } = useDecouvertes()

// ─── Compteur découvertes ────────────────────────────────────────────────

const nbDecouvertes = ref(0)

async function chargerDecouvertes() {
  try {
    const res = await listerDecouvertes()
    if (res.success && res.data) {
      nbDecouvertes.value = res.data.en_attente.total + res.data.en_cours.total
    }
  } catch {}
}

onMounted(chargerDecouvertes)

// ─── État ─────────────────────────────────────────────────────────────────

const liste = ref<PersonneListe | null>(null)
const chargement = ref(true)
const page = ref(1)
const parPage = 12
const recherche = ref('')
const rechercheDebounce = ref('')
let timerId: ReturnType<typeof setTimeout> | null = null

const modeAjout = ref<'wizard' | 'classique' | null>(null)
const creationEnCours = ref(false)
const decouverteOuverte = ref(false)

// ─── Debounce recherche ───────────────────────────────────────────────────

watch(recherche, (val) => {
  if (timerId) clearTimeout(timerId)
  timerId = setTimeout(() => {
    rechercheDebounce.value = val
    page.value = 1
  }, 350)
})

// ─── Chargement ────────────────────────────────────────────────────────────

async function charger() {
  chargement.value = true
  try {
    const res = await listerPersonnes({
      page: page.value,
      par_page: parPage,
      recherche: rechercheDebounce.value || undefined,
    })
    if (res.success && res.data) {
      liste.value = res.data
    }
  } finally {
    chargement.value = false
  }
}

// `immediate` côté CLIENT seulement. La page est derrière `middleware: 'auth'`,
// mais ce middleware laisse passer le SSR (le jeton vit dans localStorage) :
// charger la liste au rendu serveur remontait donc un 401 non rattrapé, et le
// visiteur anonyme recevait une page d'erreur au lieu d'être renvoyé vers
// /login. Le squelette est rendu côté serveur, la liste arrive côté client.
watch([page, rechercheDebounce], charger, { immediate: import.meta.client })

// ─── Navigation ────────────────────────────────────────────────────────────

const router = useRouter()

function ouvrirFiche(id: string) {
  router.push(`/arbre-genealogique/${id}`)
}

// ─── Création ─────────────────────────────────────────────────────────────

async function ajouterPersonne(form: CreerPersonneForm) {
  creationEnCours.value = true
  try {
    const res = await creerPersonne(form)
    if (res.success && res.data) {
      modeAjout.value = null
      await charger()
    }
  } catch (e: any) {
    // Si 401, tenter un refresh du token et réessayer
    if (e?.statusCode === 401 || e?.status === 401) {
      try {
        const { refreshAccessToken } = useAuth()
        await refreshAccessToken()
        const res = await creerPersonne(form)
        if (res.success && res.data) {
          modeAjout.value = null
          await charger()
        }
      } catch {
        navigateTo('/login?redirect=/arbre-genealogique')
      }
    }
  } finally {
    creationEnCours.value = false
  }
}

// ─── Pagination ────────────────────────────────────────────────────────────

const totalPages = computed(() => liste.value?.total_pages ?? 1)
const infoPagination = computed(() => {
  if (!liste.value) return ''
  const debut = (page.value - 1) * parPage + 1
  const fin = Math.min(page.value * parPage, liste.value.total)
  return `${debut}–${fin} sur ${liste.value.total}`
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Rootstree"
        sous-titre="Construire, préserver et transmettre votre héritage familial"
        aide="C'est quoi Rootstree ?"
        @aide="decouverteOuverte = true"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Rootstree' }]">
        <template #action>
          <AfricansBouton icone="fa-solid fa-plus" @click="modeAjout = 'wizard'">
            Ajouter une personne
          </AfricansBouton>
        </template>
      </AfricansFilAriane>
    </template>

    <div class="flex flex-col gap-6">
      <div class="flex flex-wrap items-baseline justify-between gap-3">
        <h2 class="text-[20px]/[1.4] font-bold text-af-chocolat">Mon arbre</h2>
        <p v-if="liste && liste.total > 0" class="text-[14px]/[1.4] text-af-atone">
          {{ liste.total }} personne{{ liste.total > 1 ? 's' : '' }} enregistrée{{ liste.total > 1 ? 's' : '' }}
        </p>
      </div>

      <label class="relative block">
        <span class="sr-only">Rechercher une personne</span>
        <font-awesome-icon
          icon="fa-solid fa-magnifying-glass"
          class="pointer-events-none absolute top-1/2 left-4 -translate-y-1/2 text-af-atone-2"
        />
        <input
          v-model="recherche"
          type="search"
          placeholder="Rechercher par nom ou prénom…"
          class="h-11 w-full rounded-[10px] border border-af-bordure bg-white pr-4 pl-11 text-[14px]/[1.4] placeholder:text-af-atone-2 focus:outline-2 focus:outline-af-chocolat"
        />
      </label>

      <!-- Chargement : autant de squelettes que de cartes par page visible. -->
      <div v-if="chargement" class="grid gap-4 sm:grid-cols-2">
        <div v-for="i in 6" :key="i" class="h-24 animate-pulse rounded-[10px] bg-af-bordure" />
      </div>

      <template v-else-if="liste && liste.personnes.length > 0">
        <div class="grid gap-4 sm:grid-cols-2">
          <PersonneCard
            v-for="personne in liste.personnes"
            :key="personne.id"
            :personne="personne"
            @click="ouvrirFiche"
          />
        </div>

        <div v-if="totalPages > 1" class="flex flex-wrap items-center justify-between gap-4">
          <p class="text-[14px]/[1.4] text-af-atone">{{ infoPagination }}</p>
          <div class="flex items-center gap-3">
            <AfricansBouton variante="secondaire" :desactive="page <= 1" @click="page--">
              Précédent
            </AfricansBouton>
            <span class="text-[14px]/[1.4] text-af-corps">{{ page }} / {{ totalPages }}</span>
            <AfricansBouton variante="secondaire" :desactive="page >= totalPages" @click="page++">
              Suivant
            </AfricansBouton>
          </div>
        </div>
      </template>

      <div v-else class="rounded-[10px] border border-af-bordure bg-white p-12 text-center">
        <font-awesome-icon icon="fa-solid fa-users" class="text-4xl text-af-atone-2" />
        <p class="mt-4 text-[16px]/[1.4] font-bold">
          {{ recherche ? 'Aucune personne trouvée' : 'Votre arbre est vide' }}
        </p>
        <p class="mt-2 text-[14px]/[1.4] text-af-corps">
          {{ recherche
            ? `Aucun résultat pour « ${recherche} »`
            : 'Ajoutez votre première personne pour commencer à construire votre arbre généalogique.'
          }}
        </p>
        <AfricansBouton v-if="!recherche" class="mt-6" icone="fa-solid fa-plus" @click="modeAjout = 'wizard'">
          Ajouter ma première personne
        </AfricansBouton>
      </div>
    </div>

    <template #rail>
      <AfricansPanneau titre="Mon espace" icone="fa-solid fa-sitemap">
        <div class="flex flex-col gap-3">
          <AfricansBouton variante="secondaire" icone="fa-solid fa-diagram-project" vers="/arbre-genealogique/visualisation">
            Voir mon arbre
          </AfricansBouton>

          <!-- Le compteur porte les découvertes EN ATTENTE et EN COURS : ce sont
               celles qui appellent une action, pas l'historique. -->
          <AfricansBouton variante="secondaire" icone="fa-solid fa-user-check" vers="/arbre-genealogique/decouvertes">
            Découvertes<span v-if="nbDecouvertes > 0"> ({{ nbDecouvertes }})</span>
          </AfricansBouton>
        </div>
      </AfricansPanneau>
    </template>

    <!-- Assistant conversationnel : le chemin par défaut de l'ajout. -->
    <AssistantAjoutPersonne
      v-if="modeAjout === 'wizard'"
      :loading="creationEnCours"
      @submit="ajouterPersonne"
      @annuler="modeAjout = null"
      @formulaire-classique="modeAjout = 'classique'"
    />

    <AfricansModale
      :model-value="modeAjout === 'classique'"
      titre="Nouvelle personne"
      @update:model-value="modeAjout = null"
    >
      <PersonneForm
        :loading="creationEnCours"
        @submit="ajouterPersonne"
        @annuler="modeAjout = null"
      />
    </AfricansModale>

    <ArbreGenealogiqueDecouverteModale v-model="decouverteOuverte" />
  </NuxtLayout>
</template>
