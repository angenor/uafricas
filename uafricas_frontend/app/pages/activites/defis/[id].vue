<script setup lang="ts">
/**
 * Résultats d'un défi (feature 013) : participants, podium, rang du membre.
 * Consultable sans connexion, et une fois la période passée.
 */
import { dureeRestante, messageErreurJeu, type ResultatsDefiAPI } from '~/composables/useJeu'

definePageMeta({ layout: false })

const route = useRoute()
const id = computed(() => String(route.params.id))

const { resultatsDefi, jouerDefi } = useJeu()
const { redirigerVersConnexion } = useAuth()
const userStore = useUserStore()

const resultats = ref<ResultatsDefiAPI | null>(null)
const chargement = ref(true)
const erreur = ref('')
const lancement = ref(false)

const titre = computed(() => {
  const d = resultats.value?.defi
  if (!d) return 'Défi'
  const jour = new Date(`${d.periode_debut}T00:00:00Z`).toLocaleDateString('fr-FR', {
    day: 'numeric', month: 'long', timeZone: 'UTC',
  })
  return d.periodicite === 'jour' ? `Défi du ${jour}` : `Défi de la semaine du ${jour}`
})

useHead(() => ({ title: `${titre.value} | Activités | AfricanS` }))

/** Le membre peut-il encore jouer ou reprendre ce défi ? */
const jouable = computed(() => {
  const r = resultats.value
  if (!r) return false
  const partie = r.defi.ma_partie
  return (r.en_cours && !partie) || partie?.etat === 'en_cours'
})

const duree = (ms: number) => {
  const s = Math.round(ms / 1000)
  return s >= 60 ? `${Math.floor(s / 60)} min ${String(s % 60).padStart(2, '0')}` : `${s} s`
}

const jouer = async () => {
  if (!userStore.isAuthenticated) {
    redirigerVersConnexion()
    return
  }
  lancement.value = true
  try {
    const res = await jouerDefi(id.value)
    if (res) await navigateTo(`/activites/partie/${res.partie_id}`)
  }
  catch (e) {
    erreur.value = messageErreurJeu(e, 'Le défi n\'a pas pu être lancé.')
  }
  finally {
    lancement.value = false
  }
}

onMounted(async () => {
  try {
    resultats.value = await resultatsDefi(id.value)
  }
  catch (e) {
    erreur.value = messageErreurJeu(e, 'Ce défi est introuvable.')
  }
  finally {
    chargement.value = false
  }
})
</script>

<template>
  <NuxtLayout name="africans">
    <template #fil-ariane>
      <AfricansFilAriane :segments="[{ libelle: 'Activités', vers: '/activites' }, { libelle: titre }]" />
    </template>

    <div class="mx-auto flex w-full max-w-3xl flex-col gap-6 pb-24">
      <div v-if="chargement" class="h-64 animate-pulse rounded-[10px] bg-af-bordure" />

      <p
        v-else-if="!resultats"
        class="rounded-[10px] border border-af-bordure bg-af-surface p-8 text-center text-[14px]/[1.5] text-af-corps"
      >
        {{ erreur || 'Ce défi est introuvable.' }}
      </p>

      <template v-else>
        <header class="rounded-[10px] border border-af-bordure bg-af-surface p-6">
          <h1 class="text-[24px]/[1.3] font-bold text-af-encre">{{ titre }}</h1>
          <p v-if="resultats.defi.titre" class="mt-1 text-[15px]/[1.5] text-af-corps">{{ resultats.defi.titre }}</p>
          <p class="mt-2 text-[14px]/[1.5] text-af-atone">
            {{ resultats.defi.nombre_epreuves }} épreuves ·
            {{ resultats.participants }} participant{{ resultats.participants > 1 ? 's' : '' }} ·
            <template v-if="resultats.en_cours">se termine dans {{ dureeRestante(resultats.defi.fin_at) }}</template>
            <template v-else>terminé</template>
          </p>

          <p v-if="erreur" class="mt-4 text-[14px]/[1.4] text-af-live" role="alert">{{ erreur }}</p>

          <AfricansBouton
            v-if="jouable"
            class="mt-5"
            icone="fa-solid fa-play"
            :desactive="lancement"
            @click="jouer"
          >
            {{ resultats.defi.ma_partie ? 'Reprendre' : 'Relever le défi' }}
          </AfricansBouton>
        </header>

        <!-- Le rang du membre, qu'il soit ou non sur le podium -->
        <section
          v-if="resultats.moi"
          class="flex flex-wrap items-center gap-4 rounded-[10px] border border-af-chocolat/40 bg-af-chocolat/[0.06] p-5"
        >
          <span class="text-[28px]/[1] font-bold text-af-chocolat">{{ resultats.moi.rang }}<sup class="text-[14px]">{{ resultats.moi.rang === 1 ? 'er' : 'e' }}</sup></span>
          <p class="text-[15px]/[1.5] text-af-encre">
            Votre résultat : <strong>{{ resultats.moi.bonnes }} / {{ resultats.defi.nombre_epreuves }}</strong>
            en {{ duree(resultats.moi.temps_total_ms) }}, sur {{ resultats.participants }}
            participant{{ resultats.participants > 1 ? 's' : '' }}.
          </p>
        </section>

        <section class="rounded-[10px] border border-af-bordure bg-af-surface">
          <h2 class="px-5 pt-5 pb-3 text-[17px]/[1.4] font-bold text-af-encre">Classement du défi</h2>

          <p v-if="resultats.podium.length === 0" class="px-5 pb-6 text-[14px]/[1.5] text-af-corps">
            Personne n'a encore terminé ce défi.
          </p>

          <ol v-else class="divide-y divide-af-bordure">
            <li
              v-for="ligne in resultats.podium"
              :key="ligne.utilisateur_id"
              class="flex items-center gap-4 px-5 py-3"
              :class="ligne.utilisateur_id === resultats.moi?.utilisateur_id && 'bg-af-chocolat/[0.06]'"
            >
              <span class="w-7 shrink-0 text-center text-[15px] font-bold" :class="ligne.rang <= 3 ? 'text-af-chocolat' : 'text-af-atone'">
                {{ ligne.rang }}
              </span>
              <AfricansAvatar :nom="`${ligne.prenom} ${ligne.nom}`" :src="urlMedia(ligne.photo_url) ?? undefined" :taille="36" />
              <NuxtLink
                :to="`/profil/${ligne.utilisateur_id}`"
                class="min-w-0 flex-1 truncate text-[15px]/[1.4] font-bold text-af-encre hover:text-af-chocolat"
              >
                {{ ligne.prenom }} {{ ligne.nom }}
              </NuxtLink>
              <span class="shrink-0 text-[14px]/[1.4] tabular-nums text-af-corps">
                <strong class="text-af-encre">{{ ligne.bonnes }}</strong> / {{ resultats.defi.nombre_epreuves }}
                <span class="ml-2 text-af-atone">{{ duree(ligne.temps_total_ms) }}</span>
              </span>
            </li>
          </ol>
        </section>
      </template>
    </div>
  </NuxtLayout>
</template>
