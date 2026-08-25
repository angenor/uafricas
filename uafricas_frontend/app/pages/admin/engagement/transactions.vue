<script setup lang="ts">
/**
 * Back-office : journal comptable des cadeaux et purge de fin de phase de test.
 *
 * daisyUI autorisé (Principe VI : administration seulement).
 *
 * Les totaux affichés sont calculés **sur le filtre courant**, pas sur la page :
 * un total qui ne porterait que sur 25 lignes n'aurait aucune valeur comptable.
 */
import { computed, onMounted, ref } from 'vue'
import {
  useAdminCadeaux, CONFIRMATION_PURGE,
  type AdminFiltresJournal, type AdminJournalPage, type AdminParametresMonetisation,
} from '~/composables/useAdminCadeaux'
import { formaterMontant } from '~/composables/useCadeaux'

definePageMeta({ layout: 'admin', middleware: ['admin'] })
useHead({ title: 'Journal des cadeaux, Administration' })

const { listerTransactions, obtenirParametres, purgerPhaseTest } = useAdminCadeaux()

const journal = ref<AdminJournalPage | null>(null)
const parametres = ref<AdminParametresMonetisation | null>(null)
const chargement = ref(true)
const message = ref('')
const erreur = ref('')

const TAILLE = 25
const filtres = ref<AdminFiltresJournal>({ page: 1, taille: TAILLE })

const devise = computed(() => parametres.value?.devise ?? 'XOF')
const totalPages = computed(() =>
  Math.max(1, Math.ceil((journal.value?.pagination.total ?? 0) / TAILLE)),
)

/** L'invariant que la recette doit pouvoir contrôler d'un coup d'œil (SC-009). */
const invariantOk = computed(() => {
  const t = journal.value?.totaux
  if (!t) return true
  return t.recettes_plateforme + t.cagnottes_dues === t.montant_total
})

const charger = async (page = filtres.value.page ?? 1) => {
  chargement.value = true
  erreur.value = ''
  filtres.value.page = page
  try {
    journal.value = await listerTransactions(filtres.value)
  } catch (e) {
    const data = (e as { data?: { error?: string } })?.data
    erreur.value = data?.error || 'Le journal n\'a pas pu être chargé.'
  } finally {
    chargement.value = false
  }
}

onMounted(async () => {
  parametres.value = await obtenirParametres()
  await charger(1)
})

const appliquerFiltres = () => charger(1)

const reinitialiser = () => {
  filtres.value = { page: 1, taille: TAILLE }
  charger(1)
}

const auMoinsUnFiltre = computed(() => {
  const f = filtres.value
  return !!(f.membre_id || f.sens || f.etat || f.mode || f.simule !== undefined || f.debut || f.fin)
})

// ── Purge de fin de phase de test ───────────────────────────────────────────

const purgeEnCours = ref(false)
const saisieConfirmation = ref('')

const purgeAutorisee = computed(() => parametres.value?.paiement_reel_actif === true)

/** Impact annoncé AVANT le déclenchement : une purge qui surprend est une purge ratée. */
const impactAttendu = computed(() => {
  const t = journal.value?.totaux
  if (!t) return null
  return { transactions: t.nombre_simule, montant: t.cagnottes_dues }
})

const lancerPurge = async () => {
  if (saisieConfirmation.value.trim() !== CONFIRMATION_PURGE) {
    erreur.value = `Saisissez « ${CONFIRMATION_PURGE} » pour confirmer.`
    return
  }
  if (!confirm(
    'Cette opération est IRRÉVERSIBLE : les points issus de cadeaux simulés seront '
    + 'supprimés, les soldes et statuts recalculés, les cagnottes réduites. Continuer ?',
  )) return

  purgeEnCours.value = true
  erreur.value = ''
  try {
    const res = await purgerPhaseTest()
    message.value = res
      ? `Purge effectuée, ${res.transactions_purgees} transaction(s), `
        + `${res.mouvements_supprimes} mouvement(s) supprimé(s), `
        + `${res.comptes_recalcules} compte(s) recalculé(s), `
        + `${formaterMontant(res.montant_cagnottes_annule, devise.value)} de cagnottes annulés.`
      : 'Purge effectuée.'
    saisieConfirmation.value = ''
    await charger(1)
  } catch (e) {
    const data = (e as { data?: { error?: string } })?.data
    erreur.value = data?.error || 'La purge a échoué.'
  } finally {
    purgeEnCours.value = false
  }
}

const LIBELLE_ETAT: Record<string, string> = {
  en_attente: 'En attente',
  abouti: 'Abouti',
  echoue: 'Échoué',
  expire: 'Expiré',
  purge: 'Purgé',
}

const CLASSE_ETAT: Record<string, string> = {
  en_attente: 'badge-warning',
  abouti: 'badge-success',
  echoue: 'badge-error',
  expire: 'badge-ghost',
  purge: 'badge-neutral',
}

const formaterDate = (iso: string) =>
  new Date(iso).toLocaleString('fr-FR', {
    day: '2-digit', month: '2-digit', year: 'numeric', hour: '2-digit', minute: '2-digit',
  })
</script>

<template>
  <div class="space-y-6 p-6">
    <div class="flex flex-wrap items-center justify-between gap-3">
      <h1 class="text-2xl font-bold">Journal des cadeaux</h1>
      <NuxtLink to="/admin/engagement/cadeaux" class="btn btn-sm btn-outline">
        <font-awesome-icon icon="fa-solid fa-gift" /> Catalogue &amp; paramètres
      </NuxtLink>
    </div>

    <div v-if="message" class="alert alert-success py-2">{{ message }}</div>
    <div v-if="erreur" class="alert alert-error py-2">{{ erreur }}</div>

    <!-- ─── Filtres ─── -->
    <div class="card bg-base-200">
      <div class="card-body flex-row flex-wrap items-end gap-3 py-4">
        <label class="form-control">
          <span class="label-text text-xs">État</span>
          <select v-model="filtres.etat" class="select select-sm select-bordered" @change="appliquerFiltres">
            <option :value="undefined">Tous</option>
            <option value="abouti">Abouti</option>
            <option value="en_attente">En attente</option>
            <option value="echoue">Échoué</option>
            <option value="expire">Expiré</option>
            <option value="purge">Purgé</option>
          </select>
        </label>
        <label class="form-control">
          <span class="label-text text-xs">Mode</span>
          <select v-model="filtres.mode" class="select select-sm select-bordered" @change="appliquerFiltres">
            <option :value="undefined">Tous</option>
            <option value="soutien_financier">Soutien financier</option>
            <option value="points">Cadeau en points</option>
          </select>
        </label>
        <label class="form-control">
          <span class="label-text text-xs">Paiement</span>
          <select v-model="filtres.simule" class="select select-sm select-bordered" @change="appliquerFiltres">
            <option :value="undefined">Tous</option>
            <option :value="true">Simulé</option>
            <option :value="false">Réel</option>
          </select>
        </label>
        <label class="form-control">
          <span class="label-text text-xs">Du</span>
          <input v-model="filtres.debut" type="date" class="input input-sm input-bordered" @change="appliquerFiltres">
        </label>
        <label class="form-control">
          <span class="label-text text-xs">Au</span>
          <input v-model="filtres.fin" type="date" class="input input-sm input-bordered" @change="appliquerFiltres">
        </label>
        <button v-if="auMoinsUnFiltre" class="btn btn-sm btn-ghost" @click="reinitialiser">
          <font-awesome-icon icon="fa-solid fa-xmark" /> Réinitialiser
        </button>
      </div>
    </div>

    <!-- ─── Totaux, calculés sur le filtre courant ─── -->
    <div v-if="journal" class="stats stats-vertical w-full shadow lg:stats-horizontal">
      <div class="stat">
        <div class="stat-title">Montant total</div>
        <div class="stat-value text-xl">{{ formaterMontant(journal.totaux.montant_total, devise) }}</div>
        <div class="stat-desc">{{ journal.totaux.nombre_abouti }} transaction(s) aboutie(s)</div>
      </div>
      <div class="stat">
        <div class="stat-title">Recettes plateforme</div>
        <div class="stat-value text-xl">{{ formaterMontant(journal.totaux.recettes_plateforme, devise) }}</div>
      </div>
      <div class="stat">
        <div class="stat-title">Cagnottes dues</div>
        <div class="stat-value text-xl">{{ formaterMontant(journal.totaux.cagnottes_dues, devise) }}</div>
        <div class="stat-desc" :class="invariantOk ? 'text-success' : 'text-error'">
          <font-awesome-icon :icon="invariantOk ? 'fa-solid fa-check' : 'fa-solid fa-triangle-exclamation'" />
          {{ invariantOk ? 'recettes + cagnottes = total' : 'INVARIANT ROMPU' }}
        </div>
      </div>
      <div class="stat">
        <div class="stat-title">Dont simulé</div>
        <div class="stat-value text-xl">{{ journal.totaux.nombre_simule }}</div>
        <div class="stat-desc">transactions retirées par la purge</div>
      </div>
    </div>

    <div v-if="chargement" class="loading loading-spinner" />

    <template v-else-if="journal">
      <div class="overflow-x-auto">
        <table class="table table-zebra table-sm">
          <thead>
            <tr>
              <th>Date</th><th>Offreur</th><th>Bénéficiaire</th><th>Cible</th>
              <th>Cadeau</th><th>Mode</th><th>Montant</th><th>Bénéf. / Plateforme</th>
              <th>Taux</th><th>Points</th><th>État</th><th>Référence</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="t in journal.elements" :key="t.id">
              <td class="whitespace-nowrap text-xs">{{ formaterDate(t.created_at) }}</td>
              <td class="text-xs">{{ t.offreur.nom_affiche }}</td>
              <td class="text-xs">{{ t.beneficiaire.nom_affiche }}</td>
              <td class="max-w-40 truncate text-xs" :title="t.cible.titre || t.cible.type_objet">
                {{ t.cible.titre || '-' }}
                <span class="block font-mono opacity-50">{{ t.cible.type_objet }}</span>
              </td>
              <td class="text-xs">{{ t.cadeau.libelle }}</td>
              <td class="text-xs">
                {{ t.mode === 'points' ? 'Points' : 'Soutien' }}
              </td>
              <td class="whitespace-nowrap text-xs">{{ formaterMontant(t.montant, devise) }}</td>
              <td class="whitespace-nowrap text-xs">
                {{ formaterMontant(t.part_beneficiaire, devise) }}
                <span class="opacity-50">/</span>
                {{ formaterMontant(t.part_plateforme, devise) }}
              </td>
              <!-- Taux FIGÉ sur la transaction : peut différer du taux courant. -->
              <td class="text-xs">{{ t.taux_commission }} %</td>
              <td class="text-xs">{{ t.points }}</td>
              <td>
                <span class="badge badge-sm" :class="CLASSE_ETAT[t.etat]">
                  {{ LIBELLE_ETAT[t.etat] || t.etat }}
                </span>
                <span v-if="t.simule" class="badge badge-ghost badge-sm ml-1">simulé</span>
              </td>
              <td class="font-mono text-[11px] opacity-60">{{ t.reference_paiement }}</td>
            </tr>
            <tr v-if="journal.elements.length === 0">
              <td colspan="12" class="py-6 text-center opacity-60">Aucune transaction.</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div v-if="totalPages > 1" class="flex items-center justify-between">
        <button
          class="btn btn-sm btn-ghost"
          :disabled="(filtres.page ?? 1) <= 1"
          @click="charger((filtres.page ?? 1) - 1)"
        >
          <font-awesome-icon icon="fa-solid fa-chevron-left" /> Précédent
        </button>
        <span class="text-sm opacity-60">
          Page {{ journal.pagination.page }} sur {{ totalPages }}
          {{ journal.pagination.total }} transaction(s)
        </span>
        <button
          class="btn btn-sm btn-ghost"
          :disabled="(filtres.page ?? 1) >= totalPages"
          @click="charger((filtres.page ?? 1) + 1)"
        >
          Suivant <font-awesome-icon icon="fa-solid fa-chevron-right" />
        </button>
      </div>
    </template>

    <!-- ─── Purge de fin de phase de test ─── -->
    <section class="space-y-3">
      <h2 class="text-lg font-semibold text-error">
        <font-awesome-icon icon="fa-solid fa-triangle-exclamation" />
        Purge de fin de phase de test
      </h2>

      <div class="alert alert-warning py-3 text-sm">
        <span>
          Cette opération supprime <strong>tous les points issus de cadeaux payés en
          simulation</strong>, recalcule les soldes et les statuts depuis le journal restant,
          et réduit les cagnottes d'autant. Les transactions sont <strong>conservées</strong>
          et marquées « purgées ».
          <br>
          Aucun point de <strong>j'aime</strong> ni de <strong>partage</strong> n'est touché :
          la suppression cible le motif de clé des cadeaux, jamais une plage de dates.
          L'opération est <strong>irréversible</strong> mais idempotente, la rejouer ne fait rien.
        </span>
      </div>

      <div
        v-if="!purgeAutorisee"
        class="alert alert-info py-2 text-sm"
      >
        <font-awesome-icon icon="fa-solid fa-lock" />
        <span>
          La purge n'est possible qu'une fois
          <NuxtLink to="/admin/engagement/cadeaux" class="link">« paiement réel »</NuxtLink>
          activé : purger tant que le paiement reste simulé rouvrirait aussitôt la porte
          aux points gratuits.
        </span>
      </div>

      <div v-else class="card border border-error/30 bg-base-200">
        <div class="card-body gap-3">
          <p v-if="impactAttendu" class="text-sm">
            Impact attendu sur le filtre courant :
            <strong>{{ impactAttendu.transactions }}</strong> transaction(s) simulée(s) aboutie(s),
            <strong>{{ formaterMontant(impactAttendu.montant, devise) }}</strong> de cagnottes.
          </p>
          <label class="form-control max-w-xs">
            <span class="label-text text-xs">
              Saisissez « {{ CONFIRMATION_PURGE }} » pour confirmer
            </span>
            <input
              v-model="saisieConfirmation"
              class="input input-sm input-bordered font-mono"
              :placeholder="CONFIRMATION_PURGE"
            >
          </label>
          <div class="card-actions justify-end">
            <button
              class="btn btn-sm btn-error"
              :disabled="purgeEnCours || saisieConfirmation.trim() !== CONFIRMATION_PURGE"
              @click="lancerPurge"
            >
              <span v-if="purgeEnCours" class="loading loading-spinner loading-xs" />
              Déclencher la purge
            </button>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>
