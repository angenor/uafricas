<script setup lang="ts">
/**
 * Espace d'activités d'un module (feature 013).
 *
 * La page se consulte sans connexion ; jouer mène à la connexion (FR-001).
 *
 * L'entraînement est ANNONCÉ avant de commencer (FR-016) : le serveur refuse
 * une partie libre quand le membre a joué toutes les épreuves, et c'est ce
 * refus qui ouvre la modale. On ne peut pas tomber dans un entraînement sans
 * l'avoir accepté.
 */
import { estRefusEntrainement, messageErreurJeu, type ModuleJeuAPI } from '~/composables/useJeu'

definePageMeta({ layout: false })

const route = useRoute()
const code = computed(() => String(route.params.module))

const { listerModules, creerPartie } = useJeu()
const { redirigerVersConnexion } = useAuth()
const userStore = useUserStore()

const moduleJeu = ref<ModuleJeuAPI | null>(null)
const chargement = ref(true)
const lancement = ref(false)
const erreur = ref('')
const entrainementPropose = ref(false)
const proposition = ref(false)
const messageDuel = ref('')

const defier = () => {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  proposition.value = true
}

useHead(() => ({ title: `${moduleJeu.value?.libelle ?? 'Activités'} | Activités | AfricanS` }))

onMounted(async () => {
  try {
    moduleJeu.value = (await listerModules()).find(m => m.code === code.value) ?? null
  }
  catch {
    erreur.value = 'Impossible de charger ce module pour le moment.'
  }
  finally {
    chargement.value = false
  }
})

const lancer = async (entrainement = false) => {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  erreur.value = ''
  lancement.value = true
  try {
    const partie = await creerPartie(code.value, { entrainement })
    if (partie) await navigateTo(`/activites/partie/${partie.id}`)
  }
  catch (e) {
    if (!entrainement && estRefusEntrainement(e)) {
      entrainementPropose.value = true
    }
    else {
      erreur.value = messageErreurJeu(e, 'La partie n\'a pas pu être lancée.')
    }
  }
  finally {
    lancement.value = false
  }
}

const accepterEntrainement = async () => {
  entrainementPropose.value = false
  await lancer(true)
}
</script>

<template>
  <NuxtLayout name="africans">
    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[{ libelle: 'Activités', vers: '/activites' }, { libelle: moduleJeu?.libelle ?? '…' }]"
      />
    </template>

    <div class="flex flex-col gap-6">
      <div v-if="chargement" class="h-56 animate-pulse rounded-[10px] bg-af-bordure" />

      <section
        v-else-if="!moduleJeu"
        class="rounded-[10px] border border-af-bordure bg-af-surface p-8 text-center"
      >
        <h1 class="text-[20px]/[1.4] font-bold text-af-encre">Ce module n'est pas ouvert au jeu</h1>
        <p class="mt-2 text-[14px]/[1.5] text-af-corps">
          {{ erreur || 'Il n\'existe pas, ou ses activités sont fermées pour le moment.' }}
        </p>
        <AfricansBouton class="mt-6" variante="secondaire" vers="/activites">
          Voir les activités
        </AfricansBouton>
      </section>

      <template v-else>
        <section class="rounded-[10px] border border-af-bordure bg-af-surface p-8">
          <div class="flex flex-wrap items-center gap-5">
            <span class="grid size-16 shrink-0 place-items-center rounded-full bg-af-chocolat/10 text-af-chocolat">
              <font-awesome-icon :icon="`fa-solid fa-${moduleJeu.icone || 'gamepad'}`" class="text-2xl" />
            </span>
            <div class="min-w-0 flex-1">
              <h1 class="text-[24px]/[1.3] font-bold text-af-encre">{{ moduleJeu.libelle }}</h1>
              <p class="mt-1 text-[14px]/[1.5] text-af-corps">
                <template v-if="moduleJeu.disponible">
                  {{ moduleJeu.epreuves_jouables }} épreuve{{ moduleJeu.epreuves_jouables > 1 ? 's' : '' }}
                  à jouer. Une partie en enchaîne dix, chacune en temps limité.
                </template>
                <template v-else>
                  Les épreuves de ce module arrivent bientôt.
                </template>
              </p>
            </div>
          </div>

          <p
            v-if="erreur"
            class="mt-6 flex items-center gap-2 rounded-lg border border-af-live/30 bg-af-live/5 px-4 py-3 text-[14px]/[1.4] text-af-live"
            role="alert"
          >
            <font-awesome-icon icon="fa-solid fa-circle-exclamation" />
            {{ erreur }}
          </p>

          <div class="mt-6 flex flex-wrap items-center gap-4">
            <AfricansBouton
              icone="fa-solid fa-play"
              :desactive="!moduleJeu.disponible || lancement"
              @click="lancer()"
            >
              {{ lancement ? 'Préparation…' : 'Jouer une partie' }}
            </AfricansBouton>
            <AfricansBouton
              variante="secondaire"
              icone="fa-solid fa-hand-fist"
              :desactive="!moduleJeu.disponible"
              @click="defier"
            >
              Défier un ami
            </AfricansBouton>
            <NuxtLink
              :to="moduleJeu.route"
              class="text-[14px]/[1.4] font-bold text-af-chocolat underline-offset-2 hover:underline"
            >
              Retour à {{ moduleJeu.libelle }}
            </NuxtLink>
          </div>
        </section>

        <section class="rounded-[10px] border border-af-bordure bg-af-surface p-6">
          <h2 class="text-[17px]/[1.4] font-bold text-af-encre">Comment ça marche</h2>
          <ul class="mt-3 flex flex-col gap-2 text-[14px]/[1.5] text-af-corps">
            <li class="flex gap-3">
              <font-awesome-icon icon="fa-solid fa-stopwatch" class="mt-1 text-af-chocolat" />
              Chaque épreuve est en temps limité. Le temps écoulé compte comme une absence de réponse.
            </li>
            <li class="flex gap-3">
              <font-awesome-icon icon="fa-solid fa-lightbulb" class="mt-1 text-af-chocolat" />
              Après chaque réponse, vous voyez la bonne réponse et son explication.
            </li>
            <li class="flex gap-3">
              <font-awesome-icon icon="fa-solid fa-star" class="mt-1 text-af-chocolat" />
              Une épreuve ne rapporte du score que la première fois que vous y répondez.
            </li>
          </ul>
        </section>
      </template>
    </div>

    <p v-if="messageDuel" class="sr-only" role="status">{{ messageDuel }}</p>
    <JeuProposerDuelModal
      v-model="proposition"
      :module-initial="code"
      @propose="d => { messageDuel = 'Défi envoyé.'; navigateTo(`/activites/duels/${d.id}`) }"
    />

    <AfricansModale
      v-model="entrainementPropose"
      titre="Vous avez tout joué"
      icone="fa-solid fa-trophy"
    >
      <p class="text-[15px]/[1.6] text-af-corps">
        Vous avez déjà répondu à toutes les épreuves de ce module. Vous pouvez lancer une partie
        d'<strong>entraînement</strong> : elle reprend des épreuves déjà vues et
        <strong>ne rapporte aucun score</strong>.
      </p>
      <template #actions>
        <button
          type="button"
          class="text-base font-bold text-af-chocolat transition hover:opacity-70"
          @click="entrainementPropose = false"
        >
          Plus tard
        </button>
        <AfricansBouton icone="fa-solid fa-play" @click="accepterEntrainement">
          S'entraîner
        </AfricansBouton>
      </template>
    </AfricansModale>
  </NuxtLayout>
</template>
