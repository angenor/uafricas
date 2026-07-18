<script setup lang="ts">
import type { AdminFactcheckCommentaire } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string

const {
  factcheckDetail, commentaires, loading, error,
  chargerDetail, modifier,
  chargerCommentaires, supprimerCommentaire,
} = useAdminFactcheck()

const ongletActif = ref('contenu')
const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

// Formulaire contenu
const form = reactive({
  contenu: '',
  source_originale: '',
  verdict: 'non_verifie',
  pays_id: '',
  couleur_fond: '#FFFFFF',
  etat: '',
})

const verdictBadgeClass = (verdict: string | null): string => {
  const classes: Record<string, string> = {
    vrai: 'badge-success',
    faux: 'badge-error',
    partiellement_vrai: 'badge-warning',
    trompeur: 'badge-warning',
    non_verifie: 'badge-ghost',
  }
  return classes[verdict || ''] || 'badge-ghost'
}

const verdictLabel = (verdict: string | null): string => {
  const labels: Record<string, string> = {
    vrai: 'Vrai',
    faux: 'Faux',
    partiellement_vrai: 'Partiellement vrai',
    trompeur: 'Trompeur',
    non_verifie: 'Non verifie',
  }
  return labels[verdict || ''] || verdict || 'Non defini'
}

const typeCommentaireBadgeClass = (type: string | null): string => {
  const classes: Record<string, string> = {
    soutien: 'badge-success',
    contradiction: 'badge-error',
  }
  return classes[type || ''] || 'badge-neutral'
}

const typeCommentaireLabel = (type: string | null): string => {
  const labels: Record<string, string> = {
    soutien: 'Soutien',
    contradiction: 'Contradiction',
  }
  return labels[type || ''] || type || 'General'
}

const charger = async () => {
  await chargerDetail(id)
  if (factcheckDetail.value) {
    const p = factcheckDetail.value
    form.contenu = p.contenu || ''
    form.source_originale = p.source_originale || ''
    form.verdict = p.verdict || 'non_verifie'
    form.pays_id = p.pays_id || ''
    form.couleur_fond = p.couleur_fond || '#FFFFFF'
    form.etat = p.etat
  }
}

const sauvegarderContenu = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    const body: any = {
      contenu: form.contenu.trim(),
      verdict: form.verdict,
      etat: form.etat,
    }
    if (form.source_originale.trim()) body.source_originale = form.source_originale.trim()
    if (form.pays_id.trim()) body.pays_id = form.pays_id.trim()
    if (form.couleur_fond && form.couleur_fond !== '#FFFFFF') body.couleur_fond = form.couleur_fond
    await modifier(id, body)
    successMsg.value = 'Contenu mis a jour avec succes'
    setTimeout(() => { successMsg.value = null }, 3000)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la sauvegarde'
  } finally {
    saving.value = false
  }
}

// Commentaires
const chargerComms = async () => {
  await chargerCommentaires(id)
}

const supprimerComm = async (commentaireId: string) => {
  try {
    await supprimerCommentaire(id, commentaireId)
    await chargerComms()
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
}

// Chargement par onglet
watch(ongletActif, (val) => {
  if (val === 'commentaires') chargerComms()
})

onMounted(() => charger())
</script>

<template>
  <div>
    <AdminPageHeader
      :titre="factcheckDetail ? 'FactCheck' : 'Chargement...'"
      sous-titre="Edition du factcheck"
    >
      <template #actions>
        <NuxtLink to="/admin/factcheck" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !factcheckDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="factcheckDetail">
      <!-- Infos rapides -->
      <div class="flex items-center gap-4 mb-6">
        <div class="avatar placeholder">
          <div class="bg-neutral text-neutral-content rounded-full w-16 h-16 flex items-center justify-center">
            <font-awesome-icon icon="magnifying-glass" class="text-xl" />
          </div>
        </div>
        <div>
          <h2 class="text-lg font-bold">{{ factcheckDetail.contenu.substring(0, 60) }}{{ factcheckDetail.contenu.length > 60 ? '...' : '' }}</h2>
          <p class="text-sm text-base-content/60">Par {{ factcheckDetail.cree_par_nom }} &mdash; {{ new Date(factcheckDetail.created_at).toLocaleDateString('fr-FR') }}</p>
          <div class="flex gap-2 mt-1">
            <span class="badge badge-sm" :class="verdictBadgeClass(factcheckDetail.verdict)">
              {{ verdictLabel(factcheckDetail.verdict) }}
            </span>
            <AdminStatusBadge :statut="factcheckDetail.etat" />
            <span v-if="factcheckDetail.pays_nom" class="badge badge-xs badge-outline">{{ factcheckDetail.pays_nom }}</span>
            <span class="badge badge-xs badge-outline">
              <font-awesome-icon icon="thumbs-up" class="mr-1" /> {{ factcheckDetail.nombre_likes }}
            </span>
            <span class="badge badge-xs badge-outline">
              <font-awesome-icon icon="thumbs-down" class="mr-1" /> {{ factcheckDetail.nombre_dislikes }}
            </span>
            <span class="badge badge-xs badge-outline">
              <font-awesome-icon icon="comment" class="mr-1" /> {{ factcheckDetail.nombre_commentaires }}
            </span>
          </div>
        </div>
      </div>

      <!-- Mise en avant (engagement +5) -->
      <div class="mb-4">
        <AdminMiseEnAvantBouton type-objet="factcheck" :objet-id="id" />
      </div>

      <!-- Alertes -->
      <div v-if="erreurLocale || error" class="alert alert-error mb-4">
        <font-awesome-icon icon="circle-exclamation" />
        <span>{{ erreurLocale || error }}</span>
        <button class="btn btn-ghost btn-xs" @click="erreurLocale = null">
          <font-awesome-icon icon="xmark" />
        </button>
      </div>
      <div v-if="successMsg" class="alert alert-success mb-4">
        <font-awesome-icon icon="circle-check" />
        <span>{{ successMsg }}</span>
      </div>

      <!-- Onglets -->
      <div role="tablist" class="tabs tabs-bordered mb-6">
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'contenu' }" @click="ongletActif = 'contenu'">
          <font-awesome-icon icon="pen" class="mr-1" /> Contenu
        </button>
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'commentaires' }" @click="ongletActif = 'commentaires'">
          <font-awesome-icon icon="comments" class="mr-1" /> Commentaires
        </button>
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'reactions' }" @click="ongletActif = 'reactions'">
          <font-awesome-icon icon="heart" class="mr-1" /> Reactions
        </button>
      </div>

      <!-- Onglet Contenu -->
      <div v-if="ongletActif === 'contenu'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <form @submit.prevent="sauvegarderContenu" class="space-y-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Contenu *</span></label>
              <textarea v-model="form.contenu" class="textarea textarea-bordered" rows="4" required />
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Source originale (URL)</span></label>
              <input v-model="form.source_originale" type="url" class="input input-bordered" placeholder="https://...">
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Verdict</span></label>
                <select v-model="form.verdict" class="select select-bordered">
                  <option value="vrai">Vrai</option>
                  <option value="faux">Faux</option>
                  <option value="partiellement_vrai">Partiellement vrai</option>
                  <option value="trompeur">Trompeur</option>
                  <option value="non_verifie">Non verifie</option>
                </select>
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Etat</span></label>
                <select v-model="form.etat" class="select select-bordered">
                  <option value="publie">Publie</option>
                  <option value="brouillon">Brouillon</option>
                  <option value="suspendu">Suspendu</option>
                </select>
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Territoire (UUID)</span></label>
                <input v-model="form.pays_id" type="text" class="input input-bordered" placeholder="ID du territoire">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Couleur de fond</span></label>
                <div class="flex items-center gap-3">
                  <input v-model="form.couleur_fond" type="color" class="w-12 h-10 rounded cursor-pointer border border-base-300">
                  <input v-model="form.couleur_fond" type="text" class="input input-bordered input-sm w-32" placeholder="#FFFFFF">
                </div>
              </div>
            </div>

            <div class="flex justify-end pt-2">
              <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
                <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" />
                Enregistrer
              </button>
            </div>
          </form>
        </div>
      </div>

      <!-- Onglet Commentaires -->
      <div v-if="ongletActif === 'commentaires'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="font-semibold mb-4">Commentaires (moderation)</h3>

          <div v-if="!commentaires.length" class="text-center text-base-content/50 py-8">
            Aucun commentaire pour ce factcheck
          </div>

          <!-- Arbre de commentaires recursif -->
          <template v-else>
            <div v-for="comm in commentaires" :key="comm.id">
              <!-- Commentaire parent -->
              <div class="flex items-start gap-3 py-3 border-b border-base-200">
                <div class="avatar placeholder">
                  <div class="bg-base-300 text-base-content rounded-full w-8 h-8">
                    <font-awesome-icon icon="user" class="text-xs" />
                  </div>
                </div>
                <div class="flex-1 min-w-0">
                  <div class="flex items-center gap-2">
                    <span class="font-semibold text-sm">{{ comm.auteur_nom }}</span>
                    <span v-if="comm.type_commentaire" class="badge badge-xs" :class="typeCommentaireBadgeClass(comm.type_commentaire)">
                      {{ typeCommentaireLabel(comm.type_commentaire) }}
                    </span>
                    <span class="text-xs text-base-content/50">{{ new Date(comm.created_at).toLocaleDateString('fr-FR') }}</span>
                    <span v-if="comm.nombre_likes > 0" class="badge badge-xs badge-outline">
                      <font-awesome-icon icon="thumbs-up" class="mr-1" /> {{ comm.nombre_likes }}
                    </span>
                  </div>
                  <p v-if="comm.supprime" class="text-sm italic text-base-content/40 mt-1">[Commentaire supprime]</p>
                  <p v-else class="text-sm mt-1">{{ comm.contenu }}</p>
                </div>
                <button
                  v-if="!comm.supprime"
                  class="btn btn-ghost btn-xs text-error flex-shrink-0"
                  @click="supprimerComm(comm.id)"
                >
                  <font-awesome-icon icon="trash" />
                </button>
              </div>

              <!-- Enfants (niveau 1) -->
              <div v-for="enfant in comm.enfants" :key="enfant.id" class="ml-8">
                <div class="flex items-start gap-3 py-3 border-b border-base-200">
                  <div class="avatar placeholder">
                    <div class="bg-base-300 text-base-content rounded-full w-7 h-7">
                      <font-awesome-icon icon="user" class="text-xs" />
                    </div>
                  </div>
                  <div class="flex-1 min-w-0">
                    <div class="flex items-center gap-2">
                      <span class="font-semibold text-sm">{{ enfant.auteur_nom }}</span>
                      <span v-if="enfant.type_commentaire" class="badge badge-xs" :class="typeCommentaireBadgeClass(enfant.type_commentaire)">
                        {{ typeCommentaireLabel(enfant.type_commentaire) }}
                      </span>
                      <span class="text-xs text-base-content/50">{{ new Date(enfant.created_at).toLocaleDateString('fr-FR') }}</span>
                      <span v-if="enfant.nombre_likes > 0" class="badge badge-xs badge-outline">
                        <font-awesome-icon icon="thumbs-up" class="mr-1" /> {{ enfant.nombre_likes }}
                      </span>
                    </div>
                    <p v-if="enfant.supprime" class="text-sm italic text-base-content/40 mt-1">[Commentaire supprime]</p>
                    <p v-else class="text-sm mt-1">{{ enfant.contenu }}</p>
                  </div>
                  <button
                    v-if="!enfant.supprime"
                    class="btn btn-ghost btn-xs text-error flex-shrink-0"
                    @click="supprimerComm(enfant.id)"
                  >
                    <font-awesome-icon icon="trash" />
                  </button>
                </div>

                <!-- Enfants (niveau 2) -->
                <div v-for="sousEnfant in enfant.enfants" :key="sousEnfant.id" class="ml-8">
                  <div class="flex items-start gap-3 py-3 border-b border-base-200">
                    <div class="avatar placeholder">
                      <div class="bg-base-300 text-base-content rounded-full w-6 h-6">
                        <font-awesome-icon icon="user" class="text-[0.6rem]" />
                      </div>
                    </div>
                    <div class="flex-1 min-w-0">
                      <div class="flex items-center gap-2">
                        <span class="font-semibold text-sm">{{ sousEnfant.auteur_nom }}</span>
                        <span v-if="sousEnfant.type_commentaire" class="badge badge-xs" :class="typeCommentaireBadgeClass(sousEnfant.type_commentaire)">
                          {{ typeCommentaireLabel(sousEnfant.type_commentaire) }}
                        </span>
                        <span class="text-xs text-base-content/50">{{ new Date(sousEnfant.created_at).toLocaleDateString('fr-FR') }}</span>
                        <span v-if="sousEnfant.nombre_likes > 0" class="badge badge-xs badge-outline">
                          <font-awesome-icon icon="thumbs-up" class="mr-1" /> {{ sousEnfant.nombre_likes }}
                        </span>
                      </div>
                      <p v-if="sousEnfant.supprime" class="text-sm italic text-base-content/40 mt-1">[Commentaire supprime]</p>
                      <p v-else class="text-sm mt-1">{{ sousEnfant.contenu }}</p>
                    </div>
                    <button
                      v-if="!sousEnfant.supprime"
                      class="btn btn-ghost btn-xs text-error flex-shrink-0"
                      @click="supprimerComm(sousEnfant.id)"
                    >
                      <font-awesome-icon icon="trash" />
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </template>
        </div>
      </div>

      <!-- Onglet Reactions -->
      <div v-if="ongletActif === 'reactions'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="font-semibold mb-6">Statistiques des reactions</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div class="card bg-success/10 border border-success/20">
              <div class="card-body items-center text-center">
                <font-awesome-icon icon="thumbs-up" class="text-4xl text-success mb-2" />
                <p class="text-4xl font-bold text-success">{{ factcheckDetail.nombre_likes }}</p>
                <p class="text-sm text-base-content/60">Likes</p>
              </div>
            </div>
            <div class="card bg-error/10 border border-error/20">
              <div class="card-body items-center text-center">
                <font-awesome-icon icon="thumbs-down" class="text-4xl text-error mb-2" />
                <p class="text-4xl font-bold text-error">{{ factcheckDetail.nombre_dislikes }}</p>
                <p class="text-sm text-base-content/60">Dislikes</p>
              </div>
            </div>
          </div>
          <div class="mt-6">
            <div class="flex justify-between text-sm mb-1">
              <span>Ratio positif</span>
              <span>{{ factcheckDetail.nombre_likes + factcheckDetail.nombre_dislikes > 0 ? Math.round((factcheckDetail.nombre_likes / (factcheckDetail.nombre_likes + factcheckDetail.nombre_dislikes)) * 100) : 0 }}%</span>
            </div>
            <progress
              class="progress progress-success w-full"
              :value="factcheckDetail.nombre_likes"
              :max="factcheckDetail.nombre_likes + factcheckDetail.nombre_dislikes || 1"
            />
          </div>
        </div>
      </div>
    </template>
  </div>
</template>
