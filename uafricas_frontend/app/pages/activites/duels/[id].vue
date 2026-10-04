<script setup lang="ts">
/**
 * Un duel (feature 013), vu de l'un des deux joueurs.
 *
 * Le résultat de l'autre n'est affiché qu'une fois le duel terminé : on ne joue
 * pas en connaissant le score à battre. Un duel amical (plafond du jour
 * atteint) le dit avant qu'on joue.
 */
import { dureeRestante, messageErreurJeu } from '~/composables/useJeu'
import { resultatDuel, type DuelAPI } from '~/composables/useDuels'

definePageMeta({ layout: false, middleware: 'auth' })

const route = useRoute()
const id = computed(() => String(route.params.id))

const { obtenirDuel, accepter, refuser, annuler, jouer, convertir, signal, dernierSignal } = useDuels()
const userStore = useUserStore()
const moiId = computed(() => userStore.user?.id ?? null)

const duel = ref<DuelAPI | null>(null)
const chargement = ref(true)
const occupe = ref(false)
const erreur = ref('')

const nomAdversaire = computed(() =>
  duel.value?.adversaire ? `${duel.value.adversaire.prenom} ${duel.value.adversaire.nom}` : 'Votre adversaire',
)

useHead(() => ({ title: `Duel contre ${nomAdversaire.value} | AfricanS` }))

const jAiFini = computed(() => duel.value?.ma_partie != null && duel.value.ma_partie.etat !== 'en_cours')
const peutJouer = computed(() =>
  duel.value != null && duel.value.mode === 'differe'
  && ['accepte', 'en_cours'].includes(duel.value.etat) && !jAiFini.value,
)
/** Duel direct accepté ou en cours : on est dans la salle. */
const enSalle = computed(() =>
  duel.value?.mode === 'direct' && ['accepte', 'en_cours'].includes(duel.value.etat),
)
/** Un duel direct resté sans réponse peut être rouvert en différé par son auteur. */
const convertible = computed(() =>
  duel.value?.mode === 'direct' && duel.value.etat === 'expire' && duel.value.je_propose,
)

const rouvrirEnDiffere = async () => {
  occupe.value = true
  erreur.value = ''
  try {
    const nouveau = await convertir(id.value)
    if (nouveau) await navigateTo(`/activites/duels/${nouveau.id}`)
  }
  catch (e) {
    erreur.value = messageErreurJeu(e)
  }
  finally {
    occupe.value = false
  }
}

const statut = computed(() => {
  const d = duel.value
  if (!d) return ''
  switch (d.etat) {
    case 'propose':
      if (d.mode === 'direct') {
        return d.je_propose
          ? `Duel en direct : en attente de ${nomAdversaire.value}. Il a quelques minutes pour accepter.`
          : `${nomAdversaire.value} vous défie en direct.`
      }
      return d.je_propose ? `En attente de la réponse de ${nomAdversaire.value}.` : `${nomAdversaire.value} vous défie.`
    case 'accepte':
    case 'en_cours':
      if (d.mode === 'direct') return ''
      return jAiFini.value
        ? `Vous avez joué. Le résultat tombera quand ${nomAdversaire.value} aura joué, ou à l'échéance.`
        : 'À vous de jouer.'
    default:
      return resultatDuel(d, moiId.value)
  }
})

const duree = (ms: number) => {
  const s = Math.round(ms / 1000)
  return s >= 60 ? `${Math.floor(s / 60)} min ${String(s % 60).padStart(2, '0')}` : `${s} s`
}

const charger = async () => {
  try {
    duel.value = await obtenirDuel(id.value)
  }
  catch (e) {
    erreur.value = messageErreurJeu(e, 'Ce duel est introuvable.')
  }
  finally {
    chargement.value = false
  }
}

const agir = async (action: () => Promise<DuelAPI | null>) => {
  erreur.value = ''
  occupe.value = true
  try {
    duel.value = (await action()) ?? duel.value
  }
  catch (e) {
    erreur.value = messageErreurJeu(e)
    await charger()
  }
  finally {
    occupe.value = false
  }
}

const lancer = async () => {
  occupe.value = true
  try {
    const res = await jouer(id.value)
    if (res) await navigateTo(`/activites/partie/${res.partie_id}`)
  }
  catch (e) {
    erreur.value = messageErreurJeu(e)
    await charger()
  }
  finally {
    occupe.value = false
  }
}

// Un signal sur CE duel : on relit.
watch(signal, () => {
  if (dernierSignal.value?.duel_id === id.value) charger()
})
onMounted(charger)
</script>

<template>
  <NuxtLayout name="africans">
    <template #fil-ariane>
      <AfricansFilAriane
        :segments="[
          { libelle: 'Activités', vers: '/activites' },
          { libelle: 'Mes duels', vers: '/activites/duels' },
          { libelle: 'Duel' },
        ]"
      />
    </template>

    <div class="mx-auto flex w-full max-w-3xl flex-col gap-6 pb-24">
      <div v-if="chargement" class="h-64 animate-pulse rounded-[10px] bg-af-bordure" />

      <p
        v-else-if="!duel"
        class="rounded-[10px] border border-af-bordure bg-af-surface p-8 text-center text-[14px]/[1.5] text-af-corps"
      >
        {{ erreur || 'Ce duel est introuvable.' }}
      </p>

      <template v-else>
        <section class="rounded-[10px] border border-af-bordure bg-af-surface p-6">
          <div class="flex flex-wrap items-center gap-4">
            <AfricansAvatar :nom="nomAdversaire" :src="urlMedia(duel.adversaire?.photoUrl) ?? undefined" :taille="56" />
            <div class="min-w-0 flex-1">
              <h1 class="text-[22px]/[1.3] font-bold text-af-encre">Duel contre {{ nomAdversaire }}</h1>
              <p class="mt-1 text-[14px]/[1.5] text-af-corps">
                <template v-if="duel.mode === 'direct'"><strong class="text-af-chocolat">En direct</strong> · </template>
                {{ duel.module_libelle }}
                <template v-if="duel.nombre_epreuves"> · {{ duel.nombre_epreuves }} épreuves</template>
                <template v-if="!['termine', 'refuse', 'annule', 'expire'].includes(duel.etat)">
                  · plus que {{ dureeRestante(duel.echeance_at) }}
                </template>
              </p>
            </div>
          </div>

          <p
            v-if="!duel.compte"
            class="mt-4 flex items-center gap-2 rounded-lg border border-af-bordure bg-af-fond px-4 py-3 text-[14px]/[1.5] text-af-corps"
          >
            <font-awesome-icon icon="fa-solid fa-circle-info" class="text-af-chocolat" />
            Duel amical : le nombre de duels comptés du jour est atteint, celui-ci ne rapporte rien.
          </p>

          <p class="mt-4 text-[16px]/[1.5] font-bold" :class="duel.vainqueur_id && duel.vainqueur_id === moiId ? 'text-af-vert' : 'text-af-encre'">
            {{ statut }}
          </p>

          <p v-if="erreur" class="mt-3 text-[14px]/[1.4] text-af-live" role="alert">{{ erreur }}</p>

          <div class="mt-5 flex flex-wrap gap-3">
            <template v-if="duel.etat === 'propose' && !duel.je_propose">
              <AfricansBouton icone="fa-solid fa-check" :desactive="occupe" @click="agir(() => accepter(duel!.id))">
                Accepter
              </AfricansBouton>
              <AfricansBouton variante="secondaire" :desactive="occupe" @click="agir(() => refuser(duel!.id))">
                Refuser
              </AfricansBouton>
            </template>
            <AfricansBouton
              v-else-if="duel.etat === 'propose' && duel.je_propose"
              variante="secondaire"
              :desactive="occupe"
              @click="agir(() => annuler(duel!.id))"
            >
              Annuler le défi
            </AfricansBouton>
            <AfricansBouton
              v-if="convertible"
              icone="fa-solid fa-rotate-right"
              :desactive="occupe"
              @click="rouvrirEnDiffere"
            >
              Transformer en duel différé
            </AfricansBouton>
            <AfricansBouton v-if="peutJouer" icone="fa-solid fa-play" :desactive="occupe" @click="lancer">
              {{ duel.ma_partie ? 'Reprendre ma partie' : 'Jouer ma partie' }}
            </AfricansBouton>
          </div>
        </section>

        <!-- Duel direct : la salle, au même moment pour les deux -->
        <JeuDuelDirectSalle
          v-if="enSalle"
          :duel-id="duel.id"
          :adversaire-nom="duel.adversaire?.prenom ?? nomAdversaire"
          @termine="charger"
        />

        <!-- Résultat : les deux parties, une fois le duel terminé -->
        <section v-if="duel.etat === 'termine'" class="grid gap-4 sm:grid-cols-2">
          <div
            v-for="(partie, index) in [duel.ma_partie, duel.sa_partie]"
            :key="index"
            class="rounded-[10px] border bg-af-surface p-5"
            :class="partie && partie.utilisateur_id === duel.vainqueur_id ? 'border-af-vert' : 'border-af-bordure'"
          >
            <p class="text-[14px]/[1.4] font-bold text-af-encre">{{ index === 0 ? 'Vous' : nomAdversaire }}</p>
            <template v-if="partie">
              <p class="mt-2 text-[28px]/[1.2] font-bold text-af-encre">
                {{ partie.bonnes }}<span class="text-[15px] font-normal text-af-atone"> / {{ duel.nombre_epreuves }}</span>
              </p>
              <p class="text-[13px]/[1.4] text-af-atone">en {{ duree(partie.temps_total_ms) }}</p>
            </template>
            <p v-else class="mt-2 text-[14px]/[1.5] text-af-atone">N'a pas joué à temps.</p>
          </div>
          <p v-if="duel.mon_gain > 0" class="text-[14px]/[1.5] text-af-corps sm:col-span-2">
            Ce duel vous rapporte <strong class="text-af-vert">+{{ duel.mon_gain }}</strong> de score.
          </p>
        </section>
      </template>
    </div>
  </NuxtLayout>
</template>
