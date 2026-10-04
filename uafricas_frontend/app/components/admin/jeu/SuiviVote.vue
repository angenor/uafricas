<script setup lang="ts">
/**
 * Suivi du vote d'un concours (feature 014, US9) : volumes, équilibre des
 * présentations, classement provisoire et comptes au comportement anormal.
 * Réservé aux administrateurs : rien de tout cela n'est visible des membres
 * avant les résultats (FR-039).
 */
import { messageErreurConcours, type ConcoursAdminAPI, type SuiviVoteAPI } from '~/composables/useAdminConcours'

const props = defineProps<{ concours: ConcoursAdminAPI }>()
const { suivi, reglerVoix } = useAdminConcours()

const donnees = ref<SuiviVoteAPI | null>(null)
const erreur = ref('')
const enCours = ref(false)

const LIBELLES_MOTIF: Record<string, string> = {
  trop_rapide: 'trop rapides',
  compte_recent: 'comptes récents',
  non_verifie: 'adresses non vérifiées',
  ecartee_admin: 'écartées par l\'équipe',
}
const LIBELLES_SIGNAL: Record<string, string> = {
  rythme: 'plus de 30 votes en une minute',
  volume: 'plus de 200 votes',
  preference: 'choisit presque toujours la même photo',
}

const charger = async () => {
  erreur.value = ''
  try {
    donnees.value = await suivi(props.concours.id)
  }
  catch (e) {
    erreur.value = messageErreurConcours(e, 'Le suivi n\'a pas pu être chargé.')
  }
}

const regler = async (votant: string, action: 'ecarter' | 'retablir') => {
  if (action === 'ecarter' && !confirm('Écarter toutes les voix de ce compte dans ce concours ?')) return
  enCours.value = true
  try {
    await reglerVoix(props.concours.id, votant, action)
    await charger()
  }
  catch (e) {
    erreur.value = messageErreurConcours(e)
  }
  finally {
    enCours.value = false
  }
}

onMounted(charger)
</script>

<template>
  <div class="flex flex-col gap-6">
    <div v-if="erreur" class="alert alert-error text-sm" role="alert">{{ erreur }}</div>
    <template v-if="donnees">
      <div class="stats stats-vertical w-full shadow-sm lg:stats-horizontal">
        <div class="stat">
          <div class="stat-title">Votes</div>
          <div class="stat-value">{{ donnees.votes.total }}</div>
          <div class="stat-desc">{{ donnees.votes.comptes }} comptés · {{ donnees.votants }} votants</div>
        </div>
        <div class="stat">
          <div class="stat-title">Écartés</div>
          <div class="stat-value text-warning">{{ donnees.votes.total - donnees.votes.comptes }}</div>
          <div class="stat-desc">
            <template v-for="(n, motif) in donnees.votes.ecartes" :key="motif">{{ n }} {{ LIBELLES_MOTIF[motif] ?? motif }} · </template>
          </div>
        </div>
        <div class="stat">
          <div class="stat-title">Présentations par photo</div>
          <div class="stat-value">{{ donnees.presentations.min ?? 0 }}–{{ donnees.presentations.max ?? 0 }}</div>
          <div class="stat-desc">moyenne {{ donnees.presentations.moyenne ?? 0 }}</div>
        </div>
      </div>

      <section>
        <h3 class="mb-2 font-bold">Comptes signalés</h3>
        <p v-if="!donnees.comptes_signales.length" class="text-sm text-base-content/60">Aucun comportement anormal.</p>
        <table v-else class="table table-sm">
          <thead><tr><th>Compte</th><th>Signaux</th><th class="text-right">Votes</th><th /></tr></thead>
          <tbody>
            <tr v-for="c in donnees.comptes_signales" :key="c.utilisateur_id">
              <td>{{ c.nom }}</td>
              <td class="text-xs">{{ c.motifs.map(m => LIBELLES_SIGNAL[m]).join(' ; ') }}</td>
              <td class="text-right">{{ c.votes }}</td>
              <td class="text-right">
                <button v-if="!c.ecarte_par_admin" type="button" class="btn btn-outline btn-error btn-xs" :disabled="enCours || concours.etat !== 'actif'" @click="regler(c.utilisateur_id, 'ecarter')">
                  Écarter ses voix
                </button>
                <button v-else type="button" class="btn btn-outline btn-xs" :disabled="enCours || concours.etat !== 'actif'" @click="regler(c.utilisateur_id, 'retablir')">
                  Rétablir ses voix
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </section>

      <section>
        <h3 class="mb-2 font-bold">Classement provisoire <span class="text-sm font-normal text-base-content/60">(photos assez présentées)</span></h3>
        <p v-if="!donnees.classement_provisoire.length" class="text-sm text-base-content/60">Aucune photo n'a encore atteint le seuil de {{ concours.presentations_min }} duels comptés.</p>
        <ol v-else class="flex flex-col gap-2">
          <li v-for="f in donnees.classement_provisoire.slice(0, 10)" :key="f.participation_id" class="flex items-center gap-3">
            <span class="w-6 text-right font-bold">{{ f.rang }}</span>
            <img :src="urlMedia(f.media_url) ?? undefined" alt="" class="size-10 rounded object-cover">
            <span class="flex-1 text-sm">{{ f.auteur }}</span>
            <span class="text-sm">{{ f.taux }} % · {{ f.duels }} duels</span>
          </li>
        </ol>
      </section>
    </template>
  </div>
</template>
