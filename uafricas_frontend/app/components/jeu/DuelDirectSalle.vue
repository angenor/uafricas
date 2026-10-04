<template>
  <section class="flex flex-col gap-5">
    <!-- Bandeau : manche, scores, présence de l'adversaire -->
    <div class="flex flex-wrap items-center gap-4 rounded-[10px] border border-af-bordure bg-af-surface px-5 py-3">
      <span class="text-[14px]/[1.4] font-bold text-af-encre">
        <template v-if="etat && etat.rang > 0">Manche {{ etat.rang }} sur {{ etat.sur }}</template>
        <template v-else>Salle du duel</template>
      </span>
      <span class="ml-auto text-[15px]/[1.4] font-bold tabular-nums text-af-encre">
        Vous {{ etat?.mes_bonnes ?? 0 }} – {{ etat?.ses_bonnes ?? 0 }} {{ adversaireNom }}
      </span>
    </div>

    <p
      v-if="etat && !etat.adversaire_present && etat.phase !== 'termine'"
      class="flex items-center gap-2 rounded-[10px] border border-af-live/30 bg-af-live/5 px-4 py-3 text-[14px]/[1.5] text-af-encre"
      role="status"
    >
      <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="text-af-live" />
      {{ adversaireNom }} semble déconnecté. S'il ne revient pas à temps, vous gagnez par forfait.
    </p>

    <!-- Attente : l'autre n'est pas encore là, ou compte à rebours du départ -->
    <div
      v-if="!etat || etat.phase === 'attente'"
      class="rounded-[10px] border border-af-bordure bg-af-surface p-10 text-center"
    >
      <template v-if="departDans != null">
        <p class="text-[14px]/[1.4] text-af-atone">Départ dans</p>
        <p class="mt-2 text-[56px]/[1] font-bold tabular-nums text-af-chocolat">{{ departDans }}</p>
      </template>
      <template v-else>
        <font-awesome-icon icon="fa-solid fa-hourglass-half" class="text-3xl text-af-chocolat" />
        <p class="mt-4 text-[16px]/[1.5] text-af-encre">En attente de {{ adversaireNom }}…</p>
        <p class="mt-1 text-[14px]/[1.5] text-af-atone">Le duel commence dès que vous êtes là tous les deux.</p>
      </template>
    </div>

    <!-- Question et révélation -->
    <template v-else-if="etat.epreuve && (etat.phase === 'question' || etat.phase === 'revelation')">
      <JeuMinuteur
        v-if="etat.manche_fin_at"
        :expire-a="etat.manche_fin_at"
        :maintenant="etat.maintenant"
        :duree-s="dureeS"
        :arrete="etat.phase === 'revelation' || etat.ma_cle != null"
        @expire="relire"
      />

      <JeuCarteEpreuve
        :epreuve="etat.epreuve"
        :choix="etat.ma_cle ?? choixEnVol"
        :bonne-cle="etat.correction?.bonne_cle ?? null"
        :verrouillee="envoi || etat.ma_cle != null || etat.phase === 'revelation'"
        @choisir="repondre"
      />

      <p v-if="etat.phase === 'question' && etat.ma_cle != null" class="text-center text-[14px]/[1.5] text-af-corps">
        <template v-if="etat.adversaire_a_repondu">Vous avez répondu tous les deux : correction…</template>
        <template v-else>Réponse envoyée. En attente de {{ adversaireNom }}…</template>
      </p>
      <p v-else-if="etat.phase === 'question' && etat.adversaire_a_repondu" class="text-center text-[14px]/[1.5] text-af-chocolat">
        {{ adversaireNom }} a déjà répondu.
      </p>

      <section
        v-if="etat.phase === 'revelation' && etat.correction"
        class="rounded-[10px] border border-af-bordure bg-af-surface p-5"
        aria-live="polite"
      >
        <p class="text-[15px]/[1.5] text-af-encre">
          Vous : <strong :class="etat.correction.ma_cle === etat.correction.bonne_cle ? 'text-af-vert' : 'text-af-live'">
            {{ libelle(etat.correction.ma_cle) }}</strong>
          · {{ adversaireNom }} : <strong :class="etat.correction.sa_cle === etat.correction.bonne_cle ? 'text-af-vert' : 'text-af-live'">
            {{ libelle(etat.correction.sa_cle) }}</strong>
        </p>
        <p v-if="etat.correction.explication" class="mt-2 text-[14px]/[1.6] text-af-corps">{{ etat.correction.explication }}</p>
        <p v-if="suivanteDans != null" class="mt-3 text-[13px]/[1.4] text-af-atone">
          {{ etat.rang >= etat.sur ? 'Résultat' : 'Manche suivante' }} dans {{ suivanteDans }} s
        </p>
      </section>
    </template>

    <p v-if="erreur" class="text-[14px]/[1.4] text-af-live" role="alert">{{ erreur }}</p>
  </section>
</template>

<script setup lang="ts">
import { messageErreurJeu } from '~/composables/useJeu'
import type { EtatDuelDirectAPI } from '~/composables/useDuels'

/**
 * La salle d'un duel DIRECT.
 *
 * Tout l'état vient du serveur (`GET …/direct`), qui fait foi : il fixe les
 * instants de début et de fin de chaque manche, et ne révèle la correction
 * qu'une fois la manche close, aux deux joueurs à la fois. Ce composant relit
 * cet état :
 *   - à chaque signal `duel_*` de ce duel (le flux temps réel) ;
 *   - toutes les 3 secondes, au cas où un signal se perdrait (le flux n'a pas
 *     de tampon) : chaque relecture vaut aussi « je suis là » ;
 *   - pile aux instants charnières qu'il connaît (départ, fin de question,
 *     manche suivante), pour que les deux écrans changent en même temps.
 */
const props = defineProps<{
  duelId: string
  adversaireNom: string
}>()

const emit = defineEmits<{ termine: [] }>()

const { etatDirect, repondreDirect, signal, dernierSignal, duelDirectEnCours } = useDuels()

const SONDAGE_MS = 3000
const dureeS = 30

const etat = ref<EtatDuelDirectAPI | null>(null)
const envoi = ref(false)
const choixEnVol = ref<number | null>(null)
const erreur = ref('')

/** Écart entre l'horloge du serveur et la nôtre, recalé à chaque lecture. */
let decalageMs = 0
const horloge = ref(Date.now())
const maintenantServeur = () => horloge.value + decalageMs

let sondage: ReturnType<typeof setInterval> | null = null
let tic: ReturnType<typeof setInterval> | null = null
let rendezVous: ReturnType<typeof setTimeout> | null = null

const secondesAvant = (iso: string | null | undefined) => {
  if (!iso) return null
  const ms = new Date(iso).getTime() - maintenantServeur()
  return ms > 0 ? Math.ceil(ms / 1000) : null
}
const departDans = computed(() => (etat.value?.phase === 'attente' ? secondesAvant(etat.value.manche_debut_at) : null))
const suivanteDans = computed(() => secondesAvant(etat.value?.prochaine_at))

const libelle = (cle: number | null) =>
  cle == null ? 'pas de réponse' : (etat.value?.epreuve?.propositions.find(p => p.cle === cle)?.texte ?? '—')

/** Relit l'état et prend rendez-vous pour le prochain instant charnière. */
const relire = async () => {
  try {
    const e = await etatDirect(props.duelId)
    if (!e) return
    decalageMs = new Date(e.maintenant).getTime() - Date.now()
    // Nouvelle manche, ou plus de question ouverte : le choix en vol est périmé.
    if (e.rang !== etat.value?.rang || e.phase !== 'question') choixEnVol.value = null
    etat.value = e
    if (e.phase === 'termine') {
      arreter()
      emit('termine')
      return
    }
    programmer(e)
  }
  catch (err) {
    erreur.value = messageErreurJeu(err, 'La salle ne répond pas : nouvel essai dans un instant.')
  }
}

const programmer = (e: EtatDuelDirectAPI) => {
  if (rendezVous) clearTimeout(rendezVous)
  const cible = e.phase === 'attente'
    ? e.manche_debut_at
    : e.phase === 'question' ? e.manche_fin_at : e.prochaine_at
  if (!cible) return
  // Un rien après l'instant serveur, pour que la relecture le trouve franchi.
  const dans = new Date(cible).getTime() - maintenantServeur() + 150
  if (dans > 0 && dans < 120_000) rendezVous = setTimeout(relire, dans)
}

const repondre = async (cle: number) => {
  if (!etat.value || etat.value.phase !== 'question' || etat.value.ma_cle != null || envoi.value) return
  envoi.value = true
  choixEnVol.value = cle
  erreur.value = ''
  try {
    await repondreDirect(props.duelId, etat.value.rang, cle)
    await relire()
  }
  catch (err) {
    choixEnVol.value = null
    erreur.value = messageErreurJeu(err, 'Votre réponse n\'a pas pu être envoyée.')
    await relire()
  }
  finally {
    envoi.value = false
  }
}

const arreter = () => {
  if (sondage) clearInterval(sondage)
  if (tic) clearInterval(tic)
  if (rendezVous) clearTimeout(rendezVous)
  sondage = tic = rendezVous = null
}

// Un signal sur CE duel : relire tout de suite.
watch(signal, () => {
  if (dernierSignal.value?.duel_id === props.duelId) relire()
})

onMounted(() => {
  duelDirectEnCours.value = props.duelId
  relire()
  sondage = setInterval(relire, SONDAGE_MS)
  tic = setInterval(() => { horloge.value = Date.now() }, 250)
})
onBeforeUnmount(() => {
  arreter()
  if (duelDirectEnCours.value === props.duelId) duelDirectEnCours.value = null
})
</script>
