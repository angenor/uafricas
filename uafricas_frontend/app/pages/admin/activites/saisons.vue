<script setup lang="ts">
/**
 * Saisons du Championship (feature 013).
 *
 * L'état d'une saison se déduit de ses dates : elle s'ouvre et se clôt seule,
 * sans intervention le jour même. Deux saisons ne peuvent pas se chevaucher
 * (c'est la base qui le refuse). Hors saison, on joue quand même : le score
 * s'ajoute au total des membres, sans classement.
 */
import { messageErreurAdminJeu, type SaisonAdminAPI } from '~/composables/useAdminJeu'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { listerSaisons, creerSaison, modifierSaison, cloreSaison } = useAdminJeu()

const saisons = ref<SaisonAdminAPI[]>([])
const chargement = ref(true)
const enCours = ref(false)
const erreur = ref('')
const message = ref('')

/** `null` : formulaire fermé ; `'nouvelle'` : création ; sinon l'id édité. */
const edition = ref<string | null>(null)
const form = reactive({ nom: '', debut: '', fin: '' })

const LIBELLES_ETAT: Record<SaisonAdminAPI['etat'], string> = {
  a_venir: 'À venir',
  en_cours: 'En cours',
  close: 'Close',
}
const CLASSES_ETAT: Record<SaisonAdminAPI['etat'], string> = {
  a_venir: 'badge-info',
  en_cours: 'badge-success',
  close: 'badge-ghost',
}

const saisonEditee = computed(() => saisons.value.find(s => s.id === edition.value) ?? null)
/** Le début d'une saison commencée n'est plus modifiable : des gains la portent déjà. */
const debutFige = computed(() => saisonEditee.value?.etat === 'en_cours')

// Les champs `datetime-local` parlent en heure LOCALE, l'API en ISO 8601 UTC.
const versChamp = (iso: string) => {
  const d = new Date(iso)
  const p = (n: number) => String(n).padStart(2, '0')
  return `${d.getFullYear()}-${p(d.getMonth() + 1)}-${p(d.getDate())}T${p(d.getHours())}:${p(d.getMinutes())}`
}
const versIso = (champ: string) => new Date(champ).toISOString()

const dateLisible = (iso: string) =>
  new Date(iso).toLocaleString('fr-FR', { day: 'numeric', month: 'short', year: 'numeric', hour: '2-digit', minute: '2-digit' })

const notifier = (texte: string) => {
  message.value = texte
  setTimeout(() => { if (message.value === texte) message.value = '' }, 5000)
}

const charger = async () => {
  erreur.value = ''
  try {
    saisons.value = await listerSaisons()
  }
  catch (e) {
    erreur.value = messageErreurAdminJeu(e, 'Impossible de charger les saisons.')
  }
  finally {
    chargement.value = false
  }
}

const ouvrirCreation = () => {
  const debut = new Date()
  const fin = new Date()
  fin.setMonth(fin.getMonth() + 3)
  form.nom = ''
  form.debut = versChamp(debut.toISOString())
  form.fin = versChamp(fin.toISOString())
  edition.value = 'nouvelle'
}

const ouvrirEdition = (s: SaisonAdminAPI) => {
  form.nom = s.nom
  form.debut = versChamp(s.debut_at)
  form.fin = versChamp(s.fin_at)
  edition.value = s.id
}

const enregistrer = async () => {
  erreur.value = ''
  enCours.value = true
  try {
    const corps = {
      nom: form.nom,
      // Début figé : on renvoie la valeur exacte du serveur, pas celle du
      // champ, arrondie à la minute, que le serveur prendrait pour un changement.
      debut_at: debutFige.value && saisonEditee.value ? saisonEditee.value.debut_at : versIso(form.debut),
      fin_at: versIso(form.fin),
    }
    if (edition.value === 'nouvelle') {
      await creerSaison(corps)
      notifier('Saison créée.')
    }
    else if (edition.value) {
      await modifierSaison(edition.value, corps)
      notifier('Saison modifiée.')
    }
    edition.value = null
    await charger()
  }
  catch (e) {
    erreur.value = messageErreurAdminJeu(e)
  }
  finally {
    enCours.value = false
  }
}

const clore = async (s: SaisonAdminAPI) => {
  if (!confirm(`Clore « ${s.nom} » maintenant ? Son classement sera archivé et les places d'honneur distinguées. Ce n'est pas réversible.`)) return
  erreur.value = ''
  enCours.value = true
  try {
    await cloreSaison(s.id)
    notifier('Saison close : classement archivé.')
    await charger()
  }
  catch (e) {
    erreur.value = messageErreurAdminJeu(e)
  }
  finally {
    enCours.value = false
  }
}

onMounted(charger)
</script>

<template>
  <div>
    <AdminPageHeader
      titre="Saisons du Championship"
      sous-titre="Une saison s'ouvre et se clôt à ses dates, sans intervention"
    >
      <template #actions>
        <button type="button" class="btn btn-primary btn-sm" @click="ouvrirCreation">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouvelle saison
        </button>
      </template>
    </AdminPageHeader>

    <div v-if="message" class="alert alert-success mb-4 text-sm">{{ message }}</div>
    <div v-if="erreur" class="alert alert-error mb-4 text-sm" role="alert">{{ erreur }}</div>

    <section v-if="edition" class="card mb-6 bg-base-200">
      <form class="card-body gap-4" @submit.prevent="enregistrer">
        <h2 class="text-lg font-bold">{{ edition === 'nouvelle' ? 'Nouvelle saison' : 'Modifier la saison' }}</h2>
        <div class="grid gap-4 md:grid-cols-3">
          <label class="flex flex-col">
            <span class="label-text mb-1 font-medium">Nom *</span>
            <input v-model="form.nom" type="text" maxlength="120" class="input input-bordered input-sm w-full" placeholder="Saison 1" required>
          </label>
          <label class="flex flex-col">
            <span class="label-text mb-1 font-medium">Début *</span>
            <input v-model="form.debut" type="datetime-local" class="input input-bordered input-sm w-full" :disabled="debutFige" required>
          </label>
          <label class="flex flex-col">
            <span class="label-text mb-1 font-medium">Fin *</span>
            <input v-model="form.fin" type="datetime-local" class="input input-bordered input-sm w-full" required>
          </label>
        </div>
        <p class="text-xs text-base-content/60">
          <template v-if="debutFige">Le début d'une saison commencée ne se modifie plus.</template>
          <template v-else>Un début dans le passé est ramené à maintenant : rien de ce qui a été joué avant ne compte pour la saison.</template>
        </p>
        <div class="flex justify-end gap-2">
          <button type="button" class="btn btn-ghost btn-sm" @click="edition = null">Annuler</button>
          <button type="submit" class="btn btn-primary btn-sm" :disabled="enCours">Enregistrer</button>
        </div>
      </form>
    </section>

    <div v-if="chargement" class="flex justify-center py-16">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <div v-else class="card overflow-x-auto bg-base-100 shadow-sm">
      <table class="table table-zebra table-sm">
        <thead>
          <tr>
            <th>Saison</th>
            <th>État</th>
            <th>Début</th>
            <th>Fin</th>
            <th class="text-center">Joueurs classés</th>
            <th />
          </tr>
        </thead>
        <tbody>
          <tr v-if="saisons.length === 0">
            <td colspan="6" class="py-8 text-center text-base-content/60">
              Aucune saison. Sans saison, les membres jouent mais aucun classement ne se remplit.
            </td>
          </tr>
          <tr v-for="s in saisons" :key="s.id">
            <td class="font-medium">{{ s.nom }}</td>
            <td><span class="badge badge-sm" :class="CLASSES_ETAT[s.etat]">{{ LIBELLES_ETAT[s.etat] }}</span></td>
            <td class="whitespace-nowrap">{{ dateLisible(s.debut_at) }}</td>
            <td class="whitespace-nowrap">{{ dateLisible(s.fin_at) }}</td>
            <td class="text-center tabular-nums">{{ s.joueurs }}</td>
            <td class="whitespace-nowrap text-right">
              <NuxtLink :to="`/activites/championship`" class="btn btn-ghost btn-xs" target="_blank">Classement</NuxtLink>
              <button v-if="s.etat !== 'close'" type="button" class="btn btn-ghost btn-xs" @click="ouvrirEdition(s)">
                Modifier
              </button>
              <button
                v-if="s.etat === 'en_cours'"
                type="button"
                class="btn btn-ghost btn-xs text-error"
                :disabled="enCours"
                @click="clore(s)"
              >
                Clore
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
