<script setup lang="ts">
/**
 * Concours (feature 014). La phase d'un concours se déduit de ses dates : il
 * ouvre son appel, son vote et ses résultats seul, sans intervention le jour
 * même. Lire cette liste fait avancer le cycle de chacun.
 */
import {
  LIBELLES_PHASE,
  messageErreurConcours,
  type ConcoursAdminAPI,
  type PhaseConcours,
} from '~/composables/useAdminConcours'
import { libelleModulePlateforme } from '~/utils/modulesPlateforme'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { lister } = useAdminConcours()

const phase = ref<PhaseConcours | ''>('')
const concours = ref<ConcoursAdminAPI[]>([])
const chargement = ref(true)
const erreur = ref('')

const CLASSES_PHASE: Record<PhaseConcours, string> = {
  a_venir: 'badge-info',
  appel: 'badge-warning',
  vote: 'badge-success',
  deliberation: 'badge-secondary',
  resultats: 'badge-ghost',
  annule: 'badge-error',
}

const charger = async () => {
  erreur.value = ''
  try {
    concours.value = (await lister(phase.value))?.data ?? []
  }
  catch (e) {
    erreur.value = messageErreurConcours(e, 'Impossible de charger les concours.')
  }
  finally {
    chargement.value = false
  }
}

const dateLisible = (iso: string) =>
  new Date(iso).toLocaleString('fr-FR', { day: 'numeric', month: 'short', hour: '2-digit', minute: '2-digit' })

watch(phase, charger)
onMounted(charger)
</script>

<template>
  <div>
    <AdminPageHeader titre="Concours" sous-titre="Batailles de photos : appel, modération, vote à l'aveugle, résultats">
      <template #actions>
        <NuxtLink to="/admin/activites/concours/nouveau" class="btn btn-primary btn-sm">
          <font-awesome-icon icon="plus" class="mr-1" /> Nouveau concours
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div class="mb-4 flex items-center gap-3">
      <select v-model="phase" class="select select-bordered select-sm" aria-label="Phase">
        <option value="">Toutes les phases</option>
        <option v-for="(libelle, code) in LIBELLES_PHASE" :key="code" :value="code">{{ libelle }}</option>
      </select>
    </div>

    <div v-if="erreur" class="alert alert-error mb-4 text-sm" role="alert">{{ erreur }}</div>
    <div v-if="chargement" class="skeleton h-40 w-full" />
    <p v-else-if="!concours.length" class="py-10 text-center text-base-content/60">Aucun concours.</p>

    <div v-else class="overflow-x-auto">
      <table class="table table-sm">
        <thead>
          <tr>
            <th>Concours</th>
            <th>Phase</th>
            <th>Appel</th>
            <th>Vote</th>
            <th class="text-right">À modérer</th>
            <th class="text-right">Publiées</th>
            <th class="text-right">Votes</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="c in concours" :key="c.id" class="hover">
            <td>
              <NuxtLink :to="`/admin/activites/concours/${c.id}`" class="link font-medium">{{ c.titre }}</NuxtLink>
              <div v-if="c.rattachement" class="text-xs text-base-content/60">{{ libelleModulePlateforme(c.rattachement) }}</div>
            </td>
            <td><span class="badge badge-sm" :class="CLASSES_PHASE[c.phase]">{{ LIBELLES_PHASE[c.phase] }}</span></td>
            <td class="text-xs">{{ dateLisible(c.appel_debut) }}</td>
            <td class="text-xs">{{ dateLisible(c.vote_debut) }} → {{ dateLisible(c.vote_fin) }}</td>
            <td class="text-right">
              <span :class="c.en_attente ? 'font-bold text-warning' : ''">{{ c.en_attente }}</span>
            </td>
            <td class="text-right">{{ c.publiees }} <span class="text-xs text-base-content/50">/ {{ c.minimum_participations }} min.</span></td>
            <td class="text-right">{{ c.votes }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>
