<script setup lang="ts">
/**
 * Hub des activités ludiques (feature 013).
 *
 * Accessible sans connexion : on y découvre les modules ouverts au jeu. Les
 * blocs défis, duels et classement s'y ajoutent avec leurs paliers.
 *
 * Tailwind v4 pur : aucune classe daisyUI (Principe VI).
 */
import { messageErreurJeu, type DefiAPI, type DefisCourantsAPI, type ModuleJeuAPI } from '~/composables/useJeu'
import type { ClassementMembresAPI, SaisonAPI } from '~/composables/useChampionship'

definePageMeta({ layout: false })

useHead({ title: 'Activités | AfricanS' })

const { listerModules, defisCourants, jouerDefi } = useJeu()
const { redirigerVersConnexion } = useAuth()
const userStore = useUserStore()

const modules = ref<ModuleJeuAPI[]>([])
const decouverte = ref(false)
const chargement = ref(true)
const erreur = ref('')

// Aperçu du Championship : les premiers de la saison en cours. Un bloc de la
// page, chargé à part : son échec ne masque rien d'autre.
const { saisons: chargerSaisons, classementMembres } = useChampionship()
const saisonCourante = ref<SaisonAPI | null>(null)
const apercu = ref<ClassementMembresAPI | null>(null)

const chargerApercu = async () => {
  try {
    saisonCourante.value = (await chargerSaisons())?.courante ?? null
    if (saisonCourante.value) apercu.value = await classementMembres({ taille: 5 })
  }
  catch {
    apercu.value = null
  }
}

// Duels : ce qui attend le membre, et le bouton pour défier un ami.
const { listerDuels, signal: signalDuel } = useDuels()
const duelsEnAttente = ref<{ aRepondre: number, aJouer: number } | null>(null)
const proposition = ref(false)
const messageDuel = ref('')

const chargerDuels = async () => {
  if (!userStore.isAuthenticated) return
  try {
    const d = await listerDuels()
    duelsEnAttente.value = d ? { aRepondre: d.a_repondre.length, aJouer: d.a_jouer.length } : null
  }
  catch {
    duelsEnAttente.value = null
  }
}

const defier = () => {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  proposition.value = true
}

watch(signalDuel, chargerDuels)

const defis = ref<DefisCourantsAPI | null>(null)
const defiEnLancement = ref(false)
const erreurDefi = ref('')

const chargerDefis = async () => {
  // Les défis sont un bloc de la page, pas la page : leur échec ne doit pas
  // masquer les modules.
  try {
    defis.value = await defisCourants()
  }
  catch {
    defis.value = null
  }
}

/**
 * Ouvre la participation au défi, ou reprend celle qui existe : le serveur
 * renvoie toujours LA partie du membre sur ce défi, jamais une seconde.
 */
const relever = async (defi: DefiAPI) => {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  erreurDefi.value = ''
  defiEnLancement.value = true
  try {
    const res = await jouerDefi(defi.id)
    if (res) await navigateTo(`/activites/partie/${res.partie_id}`)
  }
  catch (e) {
    erreurDefi.value = messageErreurJeu(e, 'Le défi n\'a pas pu être lancé.')
    await chargerDefis()
  }
  finally {
    defiEnLancement.value = false
  }
}

onMounted(async () => {
  chargerDefis()
  chargerApercu()
  chargerDuels()
  try {
    modules.value = await listerModules()
  }
  catch {
    erreur.value = 'Impossible de charger les activités pour le moment.'
  }
  finally {
    chargement.value = false
  }
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #bandeau>
      <AfricansBandeauModule
        titre="Activités"
        sous-titre="Jouez, apprenez, faites gagner votre pays"
        aide="C'est quoi les Activités ?"
        @aide="decouverte = true"
      />
    </template>

    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Activités' }]" />
    </template>

    <div class="flex flex-col gap-8">
      <!-- Défis : la même série pour tous, un jour ou une semaine -->
      <section v-if="defis && (defis.jour || defis.semaine)">
        <header class="flex flex-wrap items-baseline justify-between gap-3">
          <h2 class="text-[20px]/[1.4] font-bold text-af-encre">Les défis</h2>
          <p
            v-if="defis.serie_jours > 0"
            class="inline-flex items-center gap-2 text-[14px]/[1.4] font-bold text-af-chocolat"
          >
            <font-awesome-icon icon="fa-solid fa-fire" />
            {{ defis.serie_jours }} jour{{ defis.serie_jours > 1 ? 's' : '' }} de suite
          </p>
        </header>

        <p v-if="erreurDefi" class="mt-3 text-[14px]/[1.4] text-af-live" role="alert">{{ erreurDefi }}</p>

        <div class="mt-5 grid gap-4 sm:grid-cols-2">
          <JeuCarteDefi
            v-if="defis.jour"
            :defi="defis.jour"
            :maintenant="defis.maintenant"
            :occupe="defiEnLancement"
            @jouer="relever"
          />
          <JeuCarteDefi
            v-if="defis.semaine"
            :defi="defis.semaine"
            :maintenant="defis.maintenant"
            :occupe="defiEnLancement"
            @jouer="relever"
          />
        </div>
      </section>

      <!-- Concours ouverts (feature 014) : absent s'il n'y en a aucun -->
      <JeuConcoursEnCours />

      <!-- Duels entre amis -->
      <section class="flex flex-wrap items-center gap-4 rounded-[10px] border border-af-bordure bg-af-surface p-5">
        <span class="grid size-11 shrink-0 place-items-center rounded-full bg-af-chocolat/10 text-af-chocolat">
          <font-awesome-icon icon="fa-solid fa-hand-fist" />
        </span>
        <div class="min-w-0 flex-1">
          <h2 class="text-[17px]/[1.4] font-bold text-af-encre">Duels entre amis</h2>
          <p class="text-[14px]/[1.5] text-af-corps">
            <template v-if="duelsEnAttente && (duelsEnAttente.aRepondre + duelsEnAttente.aJouer) > 0">
              <strong class="text-af-chocolat">
                {{ duelsEnAttente.aRepondre + duelsEnAttente.aJouer }} duel{{ duelsEnAttente.aRepondre + duelsEnAttente.aJouer > 1 ? 's' : '' }}
              </strong>
              vous attend{{ duelsEnAttente.aRepondre + duelsEnAttente.aJouer > 1 ? 'ent' : '' }}.
            </template>
            <template v-else>La même série, chacun quand il veut : le meilleur l'emporte.</template>
          </p>
          <p v-if="messageDuel" class="mt-1 text-[14px]/[1.5] text-af-vert">{{ messageDuel }}</p>
        </div>
        <AfricansBouton v-if="userStore.isAuthenticated" variante="secondaire" vers="/activites/duels">Mes duels</AfricansBouton>
        <AfricansBouton icone="fa-solid fa-hand-fist" @click="defier">Défier un ami</AfricansBouton>
      </section>

      <!-- Championship : aperçu de la saison en cours -->
      <section v-if="saisonCourante">
        <header class="flex flex-wrap items-baseline justify-between gap-3">
          <h2 class="text-[20px]/[1.4] font-bold text-af-encre">
            Championship <span class="font-normal text-af-atone">· {{ saisonCourante.nom }}</span>
          </h2>
          <NuxtLink
            to="/activites/championship"
            class="text-[14px]/[1.4] font-bold text-af-chocolat underline-offset-2 hover:underline"
          >
            Tout le classement
          </NuxtLink>
        </header>
        <div class="mt-5 rounded-[10px] border border-af-bordure bg-af-surface">
          <JeuTableauClassement :lignes="apercu?.elements ?? []" :moi-id="userStore.user?.id ?? null">
            <template #vide>
              Personne n'a encore de score cette saison. Soyez le premier.
            </template>
          </JeuTableauClassement>
          <p
            v-if="apercu?.moi && !apercu.elements.some(l => l.utilisateur_id === userStore.user?.id)"
            class="border-t border-af-bordure px-5 py-3 text-[14px]/[1.5] text-af-corps"
          >
            Vous êtes <strong class="text-af-chocolat">{{ apercu.moi.rang }}<sup>e</sup></strong>
            avec {{ apercu.moi.score }} de score.
          </p>
        </div>
      </section>

      <section>
        <header>
          <h2 class="text-[20px]/[1.4] font-bold text-af-encre">Choisissez un module</h2>
          <p class="mt-1 text-[14px]/[1.5] text-af-corps">
            Une partie, c'est une série d'épreuves en temps limité. Chaque bonne réponse rapporte du
            score de jeu.
          </p>
        </header>

        <div v-if="chargement" class="mt-5 grid gap-4 sm:grid-cols-2">
          <div v-for="n in 4" :key="n" class="h-[90px] animate-pulse rounded-[10px] bg-af-bordure" />
        </div>

        <p
          v-else-if="erreur"
          class="mt-5 flex items-center gap-2 rounded-[10px] border border-af-live/30 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live"
        >
          <font-awesome-icon icon="fa-solid fa-circle-exclamation" />
          {{ erreur }}
        </p>

        <p
          v-else-if="modules.length === 0"
          class="mt-5 rounded-[10px] border border-af-bordure bg-af-surface p-8 text-center text-[14px]/[1.5] text-af-corps"
        >
          Les activités ouvrent bientôt.
        </p>

        <div v-else class="mt-5 grid gap-4 sm:grid-cols-2">
          <JeuCarteModule v-for="module in modules" :key="module.code" :module="module" />
        </div>
      </section>
    </div>

    <JeuDecouverteModale v-model="decouverte" />

    <JeuProposerDuelModal
      v-model="proposition"
      @propose="d => { messageDuel = d.sera_compte ? 'Défi envoyé.' : 'Défi envoyé, en duel amical (plafond du jour atteint).'; chargerDuels() }"
    />
  </NuxtLayout>
</template>
