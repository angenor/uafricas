<script setup lang="ts">
/**
 * Joueurs des activités (feature 013) : retrouver un joueur et, si une triche
 * est établie, annuler ses gains.
 *
 * L'annulation n'efface rien : les gains sont MARQUÉS annulés dans le journal,
 * avec le motif et l'auteur, et le score du joueur est recalculé depuis ce qui
 * reste. Elle n'a pas d'inverse.
 */
import { messageErreurAdminJeu, type JoueurAdminAPI } from '~/composables/useAdminJeu'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { listerJoueurs, annulerGains } = useAdminJeu()

const joueurs = ref<JoueurAdminAPI[]>([])
const recherche = ref('')
const chargement = ref(true)
const enCours = ref(false)
const erreur = ref('')
const message = ref('')

const cible = ref<JoueurAdminAPI | null>(null)
const form = reactive({ motif: '', depuis: '', retrait: 0 })

const charger = async () => {
  erreur.value = ''
  try {
    joueurs.value = await listerJoueurs(recherche.value)
  }
  catch (e) {
    erreur.value = messageErreurAdminJeu(e, 'Impossible de charger les joueurs.')
  }
  finally {
    chargement.value = false
  }
}

const ouvrir = (j: JoueurAdminAPI) => {
  cible.value = j
  form.motif = ''
  form.depuis = ''
  form.retrait = 0
}

const confirmer = async () => {
  if (!cible.value || !form.motif.trim()) return
  const etendue = form.depuis ? `depuis le ${new Date(form.depuis).toLocaleString('fr-FR')}` : 'tous ses gains'
  if (!confirm(`Annuler ${etendue} pour ${cible.value.prenom} ${cible.value.nom} ? Cette opération n'a pas d'inverse.`)) return
  erreur.value = ''
  enCours.value = true
  try {
    const bilan = await annulerGains(cible.value.utilisateur_id, {
      motif: form.motif.trim(),
      depuis: form.depuis ? new Date(form.depuis).toISOString() : null,
      retrait_reputation: Number(form.retrait) || 0,
    })
    if (bilan) {
      message.value = `${bilan.gains_annules} gain${bilan.gains_annules > 1 ? 's' : ''} annulé${bilan.gains_annules > 1 ? 's' : ''}`
        + ` (${bilan.score_retire} de score retiré${bilan.reputation_retiree ? `, ${bilan.reputation_retiree} de réputation` : ''}).`
        + ` Nouveau score : ${bilan.score_total}. Le joueur est prévenu.`
    }
    cible.value = null
    await charger()
  }
  catch (e) {
    erreur.value = messageErreurAdminJeu(e)
  }
  finally {
    enCours.value = false
  }
}

const date = (iso: string | null) => (iso ? new Date(iso).toLocaleDateString('fr-FR') : '—')

onMounted(charger)
</script>

<template>
  <div>
    <AdminPageHeader titre="Joueurs" sous-titre="Retrouver un joueur, et sanctionner une triche établie" />

    <div v-if="message" class="alert alert-success mb-4 text-sm">{{ message }}</div>
    <div v-if="erreur" class="alert alert-error mb-4 text-sm" role="alert">{{ erreur }}</div>

    <form class="mb-4 flex flex-wrap gap-2" @submit.prevent="charger">
      <input
        v-model="recherche"
        type="search"
        class="input input-bordered input-sm w-72"
        placeholder="Nom, prénom ou e-mail…"
        aria-label="Rechercher un joueur"
      >
      <button type="submit" class="btn btn-sm">Rechercher</button>
    </form>

    <div v-if="chargement" class="flex justify-center py-16">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <div v-else class="card overflow-x-auto bg-base-100 shadow-sm">
      <table class="table table-zebra table-sm">
        <thead>
          <tr>
            <th>Joueur</th>
            <th>E-mail</th>
            <th class="text-right">Score total</th>
            <th class="text-center">Gains</th>
            <th class="text-center">Annulés</th>
            <th>Dernier gain</th>
            <th />
          </tr>
        </thead>
        <tbody>
          <tr v-if="joueurs.length === 0">
            <td colspan="7" class="py-8 text-center text-base-content/60">Aucun joueur ne correspond.</td>
          </tr>
          <tr v-for="j in joueurs" :key="j.utilisateur_id">
            <td class="font-medium">{{ j.prenom }} {{ j.nom }}</td>
            <td class="text-sm">{{ j.email }}</td>
            <td class="text-right tabular-nums">{{ j.score_total }}</td>
            <td class="text-center tabular-nums">{{ j.gains }}</td>
            <td class="text-center tabular-nums" :class="j.gains_annules > 0 && 'text-error'">{{ j.gains_annules }}</td>
            <td>{{ date(j.dernier_gain_at) }}</td>
            <td class="text-right">
              <button type="button" class="btn btn-ghost btn-xs text-error" :disabled="j.gains === 0" @click="ouvrir(j)">
                Annuler des gains
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <AdminFormModal
      :visible="cible != null"
      :titre="cible ? `Annuler des gains de ${cible.prenom} ${cible.nom}` : ''"
      mode="edition"
      taille="md"
      :loading="enCours"
      @update:visible="v => { if (!v) cible = null }"
      @soumettre="confirmer"
    >
      <div class="space-y-4">
        <label class="flex flex-col">
          <span class="label-text mb-1 font-medium">Motif *</span>
          <textarea
            v-model="form.motif"
            rows="3"
            class="textarea textarea-bordered w-full"
            placeholder="Ce qui établit la triche : il sera montré au joueur."
            required
          />
        </label>
        <label class="flex flex-col">
          <span class="label-text mb-1 font-medium">À partir du</span>
          <input v-model="form.depuis" type="datetime-local" class="input input-bordered input-sm w-full">
          <span class="mt-1 text-xs text-base-content/60">Vide : tous les gains du joueur.</span>
        </label>
        <label class="flex flex-col">
          <span class="label-text mb-1 font-medium">Réputation à retirer</span>
          <input v-model.number="form.retrait" type="number" min="0" class="input input-bordered input-sm w-32">
        </label>
        <p class="text-xs text-base-content/60">
          Les gains restent au journal, marqués annulés avec ce motif. Le score et les classements du joueur
          sont recalculés. Cette opération n'a pas d'inverse.
        </p>
      </div>
    </AdminFormModal>
  </div>
</template>
