<script setup lang="ts">
// Panneau détail + actions (Valider/Rejeter), feature 001-admin-salles-publiques, US2
import type { PropositionSalle } from '~/composables/useAfrolang'

const props = defineProps<{
  proposition: PropositionSalle
  saving?: boolean
}>()

const emit = defineEmits<{
  valider: [commentaire: string | null]
  rejeter: [commentaire: string]
}>()

const showValiderModal = ref(false)
const showRejeterModal = ref(false)
const commentaireValidation = ref('')
const commentaireRejet = ref('')
const erreurRejet = ref<string | null>(null)

const formatDateTime = (iso: string | null) =>
  iso
    ? new Date(iso).toLocaleString('fr-FR', {
        day: '2-digit', month: '2-digit', year: 'numeric',
        hour: '2-digit', minute: '2-digit',
      })
    : '-'

const statutLibelle = computed(() => {
  const libelles: Record<string, string> = {
    en_attente: 'En attente',
    validee: 'Validée',
    rejetee: 'Rejetée',
    retiree: 'Retirée',
  }
  return libelles[props.proposition.statut] || props.proposition.statut
})

const badgeClass = computed(() => {
  switch (props.proposition.statut) {
    case 'en_attente': return 'badge badge-warning'
    case 'validee': return 'badge badge-success'
    case 'rejetee': return 'badge badge-error'
    case 'retiree': return 'badge badge-neutral'
    default: return 'badge'
  }
})

const peutDecider = computed(() => props.proposition.statut === 'en_attente')

const confirmerValidation = () => {
  const c = commentaireValidation.value.trim()
  emit('valider', c.length > 0 ? c : null)
  showValiderModal.value = false
  commentaireValidation.value = ''
}

const confirmerRejet = () => {
  const c = commentaireRejet.value.trim()
  if (c.length < 10) {
    erreurRejet.value = 'Le commentaire doit contenir au moins 10 caractères.'
    return
  }
  erreurRejet.value = null
  emit('rejeter', c)
  showRejeterModal.value = false
  commentaireRejet.value = ''
}
</script>

<template>
  <div class="space-y-6">
    <!-- En-tête -->
    <div class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <div class="flex items-start justify-between gap-4">
          <div>
            <h2 class="card-title">{{ proposition.titre }}</h2>
            <p class="text-sm text-base-content/60 mt-1">
              Soumise par {{ proposition.auteur.prenom }} {{ proposition.auteur.nom }}
              le {{ formatDateTime(proposition.created_at) }}
            </p>
          </div>
          <span :class="badgeClass">{{ statutLibelle }}</span>
        </div>
      </div>
    </div>

    <!-- Détails -->
    <div class="card bg-base-100 shadow-sm">
      <div class="card-body space-y-4">
        <div>
          <h3 class="font-semibold text-sm uppercase text-base-content/70">Langue cible</h3>
          <p>{{ proposition.langue_cible }}<span v-if="proposition.langue_code"> ({{ proposition.langue_code }})</span></p>
        </div>
        <div>
          <h3 class="font-semibold text-sm uppercase text-base-content/70">Groupe ethnique</h3>
          <p>
            {{ proposition.groupe_ethnique?.nom ?? proposition.groupe_ethnique_libre }}
            <span v-if="!proposition.groupe_ethnique && proposition.groupe_ethnique_libre" class="badge badge-ghost badge-sm ml-1">Autre (texte libre)</span>
          </p>
        </div>
        <div>
          <h3 class="font-semibold text-sm uppercase text-base-content/70">Territoire d'origine</h3>
          <div class="flex flex-wrap gap-2 mt-1">
            <span
              v-for="p in proposition.pays_origine"
              :key="p.id"
              class="badge badge-outline"
            >
              {{ p.code_iso2 ? `${p.code_iso2} : ` : '' }}{{ p.nom }}
            </span>
          </div>
        </div>
        <div>
          <h3 class="font-semibold text-sm uppercase text-base-content/70">Description</h3>
          <p class="whitespace-pre-line">{{ proposition.description }}</p>
        </div>
        <div>
          <h3 class="font-semibold text-sm uppercase text-base-content/70">Justification</h3>
          <p class="whitespace-pre-line">{{ proposition.justification }}</p>
        </div>
      </div>
    </div>

    <!-- Décision (lecture seule si déjà décidée) -->
    <div v-if="!peutDecider" class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <h3 class="card-title text-base">Décision</h3>
        <div v-if="proposition.decideur" class="space-y-2 text-sm">
          <p>
            <span class="text-base-content/60">Décidée par&nbsp;:</span>
            {{ proposition.decideur.prenom }} {{ proposition.decideur.nom }}
            <span class="text-base-content/60">le {{ formatDateTime(proposition.decide_at) }}</span>
          </p>
          <p v-if="proposition.commentaire_decision">
            <span class="text-base-content/60">Commentaire&nbsp;:</span>
            <span class="whitespace-pre-line">{{ proposition.commentaire_decision }}</span>
          </p>
          <p v-if="proposition.salle_id_creee">
            <span class="text-base-content/60">Salle créée&nbsp;:</span>
            <NuxtLink
              :to="`/admin/salles/${proposition.salle_id_creee}`"
              class="link link-primary"
            >
              {{ proposition.salle_id_creee }}
            </NuxtLink>
          </p>
        </div>
        <p v-else class="text-base-content/60">
          Proposition retirée par l'auteur.
        </p>
      </div>
    </div>

    <!-- Actions -->
    <div v-if="peutDecider" class="flex justify-end gap-2">
      <button
        class="btn btn-error btn-outline"
        :disabled="saving"
        @click="showRejeterModal = true"
      >
        <font-awesome-icon icon="xmark" class="mr-1" /> Rejeter
      </button>
      <button
        class="btn btn-success"
        :disabled="saving"
        @click="showValiderModal = true"
      >
        <font-awesome-icon icon="check" class="mr-1" /> Valider et créer la salle
      </button>
    </div>

    <!-- Modal Validation -->
    <dialog v-if="showValiderModal" open class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg">Valider la proposition</h3>
        <p class="py-2 text-sm">
          Cette action crée une salle publique pour le groupe ethnique
          « {{ proposition.groupe_ethnique?.nom ?? proposition.groupe_ethnique_libre }} ». L'auteur sera notifié.
        </p>
        <div class="form-control">
          <label class="label">
            <span class="label-text">Commentaire (facultatif, transmis à l'auteur)</span>
          </label>
          <textarea
            v-model="commentaireValidation"
            class="textarea textarea-bordered"
            rows="3"
            placeholder="Ex : Bienvenue ! Salle créée."
          />
        </div>
        <div class="modal-action">
          <button class="btn btn-ghost" @click="showValiderModal = false">Annuler</button>
          <button class="btn btn-success" :disabled="saving" @click="confirmerValidation">
            Confirmer la validation
          </button>
        </div>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button @click="showValiderModal = false">close</button>
      </form>
    </dialog>

    <!-- Modal Rejet -->
    <dialog v-if="showRejeterModal" open class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg">Rejeter la proposition</h3>
        <p class="py-2 text-sm text-base-content/70">
          L'auteur recevra ce commentaire. Minimum 10 caractères.
        </p>
        <div class="form-control">
          <label class="label">
            <span class="label-text">Commentaire (obligatoire)</span>
          </label>
          <textarea
            v-model="commentaireRejet"
            class="textarea textarea-bordered"
            rows="4"
            placeholder="Ex : La langue est déjà couverte par la salle X..."
          />
        </div>
        <div v-if="erreurRejet" class="alert alert-error mt-2 text-sm">
          {{ erreurRejet }}
        </div>
        <div class="modal-action">
          <button class="btn btn-ghost" @click="showRejeterModal = false">Annuler</button>
          <button class="btn btn-error" :disabled="saving" @click="confirmerRejet">
            Confirmer le rejet
          </button>
        </div>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button @click="showRejeterModal = false">close</button>
      </form>
    </dialog>
  </div>
</template>
