<script setup lang="ts">
// Liste admin des liens externes en attente de validation (US6)
// daisyUI autorisé côté admin
import type { RessourceSalleAPI } from '~/composables/useAfrolang'

const { listerLiensEnAttente, publierLien, refuserLien } = useAdminAfrolangSalles()

const liens = ref<RessourceSalleAPI[]>([])
const chargement = ref(false)
const motifRefus = ref('')
const lienEnRefus = ref<RessourceSalleAPI | null>(null)

const recharger = async () => {
  chargement.value = true
  liens.value = await listerLiensEnAttente()
  chargement.value = false
}

const publier = async (id: string) => {
  const ok = await publierLien(id)
  if (ok) await recharger()
}

const ouvrirRefus = (lien: RessourceSalleAPI) => {
  lienEnRefus.value = lien
  motifRefus.value = ''
}

const confirmerRefus = async () => {
  if (!lienEnRefus.value || motifRefus.value.trim().length < 5) return
  const ok = await refuserLien(lienEnRefus.value.id, motifRefus.value.trim())
  if (ok) {
    lienEnRefus.value = null
    motifRefus.value = ''
    await recharger()
  }
}

onMounted(recharger)
</script>

<template>
  <div class="space-y-4">
    <div class="flex items-center justify-between">
      <h3 class="text-lg font-semibold">Liens externes à modérer</h3>
      <button type="button" class="btn btn-sm btn-ghost" @click="recharger">
        Rafraîchir
      </button>
    </div>

    <p v-if="chargement" class="text-sm text-base-content/70">Chargement...</p>
    <p v-else-if="liens.length === 0" class="text-sm text-base-content/70">
      Aucun lien en attente — tout est à jour.
    </p>

    <ul v-else class="space-y-3">
      <li
        v-for="lien in liens"
        :key="lien.id"
        class="card bg-base-200 shadow-sm"
      >
        <div class="card-body p-4 gap-2">
          <div class="flex flex-wrap items-start justify-between gap-3">
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-2">
                <span class="badge badge-warning badge-sm">En attente</span>
                <span class="text-xs text-base-content/60">
                  Ajouté par {{ lien.auteur_prenom }} {{ lien.auteur_nom }}
                </span>
              </div>
              <p class="mt-1 font-medium truncate">{{ lien.titre }}</p>
              <p v-if="lien.description" class="text-sm text-base-content/70">
                {{ lien.description }}
              </p>
              <a
                v-if="lien.lien_url"
                :href="lien.lien_url"
                target="_blank"
                rel="noopener"
                class="mt-1 inline-block text-xs text-primary break-all underline"
              >
                {{ lien.lien_url }}
              </a>
            </div>
            <div class="flex gap-2 flex-shrink-0">
              <button
                type="button"
                class="btn btn-sm btn-success"
                @click="publier(lien.id)"
              >
                Publier
              </button>
              <button
                type="button"
                class="btn btn-sm btn-error btn-outline"
                @click="ouvrirRefus(lien)"
              >
                Refuser
              </button>
            </div>
          </div>
        </div>
      </li>
    </ul>

    <!-- Modale refus -->
    <dialog :open="lienEnRefus !== null" class="modal">
      <div class="modal-box">
        <h4 class="text-lg font-bold">Refuser le lien</h4>
        <p class="py-2 text-sm">
          Motif (obligatoire, minimum 5 caractères) — sera transmis à l'auteur.
        </p>
        <textarea
          v-model="motifRefus"
          class="textarea textarea-bordered w-full"
          rows="3"
          placeholder="Motif du refus..."
        />
        <div class="modal-action">
          <button class="btn" @click="lienEnRefus = null">Annuler</button>
          <button
            class="btn btn-error"
            :disabled="motifRefus.trim().length < 5"
            @click="confirmerRefus"
          >
            Refuser
          </button>
        </div>
      </div>
    </dialog>
  </div>
</template>
