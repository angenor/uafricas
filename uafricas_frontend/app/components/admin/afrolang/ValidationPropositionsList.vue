<script setup lang="ts">
import { computed, onMounted, reactive, ref, watch } from 'vue'
import {
  useAdminAfrolangSalles,
  type ApprouverPropositionForm,
  type PropositionSalleAdminAPI,
} from '~/composables/useAdminAfrolangSalles'
import type { EtatProposition } from '~/composables/useAfrolang'
import { formatDate } from '~/composables/useAfrolang'

const admin = useAdminAfrolangSalles()
const {
  pagination,
  loading,
  listerPropositions,
  approuverProposition,
  refuserProposition,
  obtenirProposition,
} = admin

const etatFiltre = ref<EtatProposition | 'tous'>('en_attente')
const recherche = ref('')
const propositions = ref<PropositionSalleAdminAPI[]>([])

const detail = ref<PropositionSalleAdminAPI | null>(null)
const modalAction = ref<'approuver' | 'refuser' | null>(null)
const traitement = ref(false)
const messageRetour = ref<string | null>(null)

const approuverForm = reactive<ApprouverPropositionForm>({
  groupe_ethnique_id: '',
  titre: '',
  langue_code: '',
  alphabet: '',
  dictionnaire_url: '',
})
const refusMotif = ref('')

const charger = async () => {
  const resp = await listerPropositions({
    etat: etatFiltre.value,
    q: recherche.value.trim() || undefined,
  })
  propositions.value = resp?.data ?? []
}

const ouvrirDetail = async (id: string, action: 'approuver' | 'refuser') => {
  messageRetour.value = null
  modalAction.value = action
  detail.value = await obtenirProposition(id)
  if (detail.value) {
    approuverForm.groupe_ethnique_id = detail.value.groupe_ethnique_id ?? ''
    approuverForm.titre = ''
    approuverForm.langue_code = ''
    approuverForm.alphabet = ''
    approuverForm.dictionnaire_url = ''
    refusMotif.value = ''
  }
}

const fermerDetail = () => {
  detail.value = null
  modalAction.value = null
  refusMotif.value = ''
}

const confirmerApprobation = async () => {
  if (!detail.value) return
  if (!approuverForm.groupe_ethnique_id) {
    messageRetour.value = 'Le groupe ethnique cible est obligatoire.'
    return
  }
  traitement.value = true
  const result = await approuverProposition(detail.value.id, { ...approuverForm })
  traitement.value = false
  if (result) {
    messageRetour.value = 'Proposition approuvée — salle créée.'
    fermerDetail()
    await charger()
  } else {
    messageRetour.value = 'Échec de l\'approbation.'
  }
}

const confirmerRefus = async () => {
  if (!detail.value) return
  if (refusMotif.value.trim().length < 5) {
    messageRetour.value = 'Le motif doit contenir au moins 5 caractères.'
    return
  }
  traitement.value = true
  const ok = await refuserProposition(detail.value.id, refusMotif.value.trim())
  traitement.value = false
  if (ok) {
    messageRetour.value = 'Proposition refusée.'
    fermerDetail()
    await charger()
  } else {
    messageRetour.value = 'Échec du refus.'
  }
}

watch(etatFiltre, () => {
  pagination.page = 1
  charger()
})

let timer: ReturnType<typeof setTimeout> | null = null
watch(recherche, () => {
  if (timer) clearTimeout(timer)
  timer = setTimeout(() => {
    pagination.page = 1
    charger()
  }, 300)
})

onMounted(() => {
  charger()
})

const badgeEtat = (etat: EtatProposition) => {
  switch (etat) {
    case 'en_attente':
      return { classe: 'badge-warning', label: 'En attente' }
    case 'approuvee':
      return { classe: 'badge-success', label: 'Validée' }
    case 'refusee':
      return { classe: 'badge-error', label: 'Refusée' }
  }
}

const totalPages = computed(() => Math.max(1, pagination.totalPages))
</script>

<template>
  <div>
    <div class="flex flex-wrap items-end gap-3 mb-4">
      <div class="form-control">
        <label class="label py-1">
          <span class="label-text text-xs">État</span>
        </label>
        <select v-model="etatFiltre" class="select select-bordered select-sm w-48">
          <option value="en_attente">En attente</option>
          <option value="approuvee">Validées</option>
          <option value="refusee">Refusées</option>
          <option value="tous">Toutes</option>
        </select>
      </div>
      <div class="form-control flex-1 min-w-[240px]">
        <label class="label py-1">
          <span class="label-text text-xs">Recherche</span>
        </label>
        <input
          v-model="recherche"
          type="text"
          placeholder="Nom du groupe ethnique…"
          class="input input-bordered input-sm"
        />
      </div>
    </div>

    <div v-if="messageRetour" class="alert alert-info mb-4 text-sm">
      {{ messageRetour }}
    </div>

    <div v-if="loading" class="py-12 text-center text-base-content/60">
      <span class="loading loading-spinner loading-md" />
    </div>

    <div v-else-if="propositions.length === 0" class="py-12 text-center text-base-content/60">
      Aucune proposition.
    </div>

    <div v-else class="overflow-x-auto">
      <table class="table table-zebra text-sm">
        <thead>
          <tr>
            <th>Groupe</th>
            <th>Langue</th>
            <th>Proposant</th>
            <th>État</th>
            <th>Soumise le</th>
            <th class="text-right">Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="p in propositions" :key="p.id">
            <td class="font-semibold">{{ p.nom_groupe_ethnique }}</td>
            <td>{{ p.langue_cible || '—' }}</td>
            <td>
              <div class="flex flex-col">
                <span class="font-medium">{{ p.proposant_nom_complet || '—' }}</span>
                <span class="text-xs opacity-70">{{ p.proposant_email || '—' }}</span>
              </div>
            </td>
            <td>
              <span class="badge" :class="badgeEtat(p.etat as EtatProposition).classe">
                {{ badgeEtat(p.etat as EtatProposition).label }}
              </span>
            </td>
            <td>{{ formatDate(p.created_at) }}</td>
            <td class="text-right whitespace-nowrap">
              <button
                v-if="p.etat === 'en_attente'"
                class="btn btn-xs btn-success mr-1"
                @click="ouvrirDetail(p.id, 'approuver')"
              >
                Approuver
              </button>
              <button
                v-if="p.etat === 'en_attente'"
                class="btn btn-xs btn-error"
                @click="ouvrirDetail(p.id, 'refuser')"
              >
                Refuser
              </button>
              <NuxtLink
                v-if="p.etat === 'approuvee' && p.salle_id_creee"
                :to="`/afrolang/${p.salle_id_creee}`"
                class="btn btn-xs btn-ghost"
              >
                Voir la salle
              </NuxtLink>
            </td>
          </tr>
        </tbody>
      </table>

      <div v-if="totalPages > 1" class="flex justify-end gap-2 mt-4">
        <button
          class="btn btn-sm"
          :disabled="pagination.page <= 1"
          @click="(pagination.page--)"
        >
          Précédent
        </button>
        <span class="self-center text-sm">
          Page {{ pagination.page }} / {{ totalPages }}
        </span>
        <button
          class="btn btn-sm"
          :disabled="pagination.page >= totalPages"
          @click="(pagination.page++)"
        >
          Suivant
        </button>
      </div>
    </div>

    <!-- Modal Approuver -->
    <dialog v-if="modalAction === 'approuver' && detail" class="modal modal-open">
      <div class="modal-box max-w-lg">
        <h3 class="font-bold text-lg mb-2">Approuver « {{ detail.nom_groupe_ethnique }} »</h3>
        <p class="text-sm opacity-70 mb-4">
          Complétez les métadonnées pour créer la salle publique associée.
        </p>

        <div v-if="detail.salle_existante_id" class="alert alert-warning text-sm mb-3">
          Une salle existe déjà pour un nom équivalent. Vérifiez avant d'approuver.
        </div>

        <div class="form-control mb-2">
          <label class="label py-1">
            <span class="label-text">Groupe ethnique (ID) <span class="text-error">*</span></span>
          </label>
          <input
            v-model="approuverForm.groupe_ethnique_id"
            type="text"
            class="input input-bordered input-sm"
            placeholder="UUID depuis country_profile.groupe_ethnique"
          />
        </div>
        <div class="form-control mb-2">
          <label class="label py-1"><span class="label-text">Titre (optionnel)</span></label>
          <input v-model="approuverForm.titre" type="text" class="input input-bordered input-sm" />
        </div>
        <div class="grid grid-cols-2 gap-2">
          <div class="form-control">
            <label class="label py-1"><span class="label-text">Code langue</span></label>
            <input v-model="approuverForm.langue_code" type="text" class="input input-bordered input-sm" />
          </div>
          <div class="form-control">
            <label class="label py-1"><span class="label-text">Alphabet</span></label>
            <input v-model="approuverForm.alphabet" type="text" class="input input-bordered input-sm" />
          </div>
        </div>
        <div class="form-control mt-2">
          <label class="label py-1"><span class="label-text">Dictionnaire (URL)</span></label>
          <input v-model="approuverForm.dictionnaire_url" type="url" class="input input-bordered input-sm" />
        </div>

        <div class="modal-action">
          <button class="btn btn-ghost" :disabled="traitement" @click="fermerDetail">Annuler</button>
          <button
            class="btn btn-success"
            :disabled="traitement || !approuverForm.groupe_ethnique_id"
            @click="confirmerApprobation"
          >
            <span v-if="traitement" class="loading loading-spinner loading-xs" />
            Valider et créer la salle
          </button>
        </div>
      </div>
    </dialog>

    <!-- Modal Refuser -->
    <dialog v-if="modalAction === 'refuser' && detail" class="modal modal-open">
      <div class="modal-box max-w-md">
        <h3 class="font-bold text-lg mb-2">Refuser « {{ detail.nom_groupe_ethnique }} »</h3>
        <p class="text-sm opacity-70 mb-4">
          Motif transmis au proposant dans la notification de refus (≥ 5 caractères).
        </p>
        <textarea
          v-model="refusMotif"
          class="textarea textarea-bordered w-full"
          rows="4"
          placeholder="Expliquez en quelques mots la raison du refus…"
        />
        <div class="modal-action">
          <button class="btn btn-ghost" :disabled="traitement" @click="fermerDetail">Annuler</button>
          <button
            class="btn btn-error"
            :disabled="traitement || refusMotif.trim().length < 5"
            @click="confirmerRefus"
          >
            <span v-if="traitement" class="loading loading-spinner loading-xs" />
            Confirmer le refus
          </button>
        </div>
      </div>
    </dialog>
  </div>
</template>
