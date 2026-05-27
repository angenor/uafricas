<script setup lang="ts">
import type { AdminCodimoiCommentaire } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string

const {
  postDetail, commentaires, loading, error,
  chargerDetail, modifier, ajouterTag, retirerTag,
  chargerCommentaires, supprimerCommentaire,
} = useAdminCodimoi()

const ongletActif = ref('contenu')
const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

// Formulaire contenu
const form = reactive({
  type_codimoi: '',
  contenu: '',
  explication: '',
  nom_auteur_originel: '',
  pays_id: '',
  groupe_ethnique: '',
  couleur_fond: '#FFFFFF',
  etat: '',
  image_couverture_url: '',
  image_arriere_plan_url: '',
})

// Tag
const nouveauTagId = ref('')

const typeLabel = (type: string | null): string => {
  const labels: Record<string, string> = {
    proverbe_adage: 'Proverbe / Adage',
    citation: 'Citation',
    ressource_historique: 'Ressource historique',
    bonne_pratique: 'Bonne pratique',
  }
  return labels[type || ''] || type || 'Non defini'
}

const charger = async () => {
  await chargerDetail(id)
  if (postDetail.value) {
    const p = postDetail.value
    form.type_codimoi = p.type_codimoi || ''
    form.contenu = p.contenu
    form.explication = p.explication || ''
    form.nom_auteur_originel = p.nom_auteur_originel || ''
    form.pays_id = p.pays_id || ''
    form.groupe_ethnique = p.groupe_ethnique || ''
    form.couleur_fond = p.couleur_fond || '#FFFFFF'
    form.etat = p.etat
    form.image_couverture_url = p.image_couverture_url || ''
    form.image_arriere_plan_url = p.image_arriere_plan_url || ''
  }
}

const sauvegarderContenu = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    const body: any = {
      type_codimoi: form.type_codimoi,
      contenu: form.contenu.trim(),
      etat: form.etat,
    }
    if (form.explication.trim()) body.explication = form.explication.trim()
    if (form.nom_auteur_originel.trim()) body.nom_auteur_originel = form.nom_auteur_originel.trim()
    if (form.pays_id.trim()) body.pays_id = form.pays_id.trim()
    if (form.groupe_ethnique.trim()) body.groupe_ethnique = form.groupe_ethnique.trim()
    if (form.couleur_fond && form.couleur_fond !== '#FFFFFF') body.couleur_fond = form.couleur_fond
    if (form.image_couverture_url.trim()) body.image_couverture_url = form.image_couverture_url.trim()
    if (form.image_arriere_plan_url.trim()) body.image_arriere_plan_url = form.image_arriere_plan_url.trim()
    await modifier(id, body)
    successMsg.value = 'Contenu mis a jour avec succes'
    setTimeout(() => { successMsg.value = null }, 3000)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la sauvegarde'
  } finally {
    saving.value = false
  }
}

// Tags
const ajouterTagHandler = async () => {
  if (!nouveauTagId.value) return
  try {
    await ajouterTag(id, nouveauTagId.value)
    nouveauTagId.value = ''
    await chargerDetail(id)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
}

const supprimerTagHandler = async (tagId: string) => {
  try {
    await retirerTag(id, tagId)
    await chargerDetail(id)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
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
      :titre="postDetail ? typeLabel(postDetail.type_codimoi) : 'Chargement...'"
      sous-titre="Edition du codi-moi"
    >
      <template #actions>
        <NuxtLink to="/admin/codimoi" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !postDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="postDetail">
      <!-- Infos rapides -->
      <div class="flex items-center gap-4 mb-6">
        <div class="avatar placeholder">
          <div class="bg-neutral text-neutral-content rounded-full w-16 h-16 flex items-center justify-center">
            <font-awesome-icon icon="quote-left" class="text-xl" />
          </div>
        </div>
        <div>
          <h2 class="text-lg font-bold">{{ postDetail.contenu.substring(0, 60) }}{{ postDetail.contenu.length > 60 ? '...' : '' }}</h2>
          <p class="text-sm text-base-content/60">Par {{ postDetail.cree_par_nom }} &mdash; {{ new Date(postDetail.created_at).toLocaleDateString('fr-FR') }}</p>
          <div class="flex gap-2 mt-1">
            <AdminStatusBadge :statut="postDetail.etat" />
            <span v-if="postDetail.pays_nom" class="badge badge-xs badge-outline">{{ postDetail.pays_nom }}</span>
            <span class="badge badge-xs badge-outline">
              <font-awesome-icon icon="thumbs-up" class="mr-1" /> {{ postDetail.nombre_likes }}
            </span>
            <span class="badge badge-xs badge-outline">
              <font-awesome-icon icon="comment" class="mr-1" /> {{ postDetail.nombre_commentaires }}
            </span>
          </div>
        </div>
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
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'tags' }" @click="ongletActif = 'tags'">
          <font-awesome-icon icon="tags" class="mr-1" /> Tags
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
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Type *</span></label>
                <select v-model="form.type_codimoi" class="select select-bordered" required>
                  <option value="proverbe_adage">Proverbe / Adage</option>
                  <option value="citation">Citation</option>
                  <option value="ressource_historique">Ressource historique</option>
                  <option value="bonne_pratique">Bonne pratique</option>
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

            <div class="form-control">
              <label class="label"><span class="label-text">Contenu *</span></label>
              <textarea v-model="form.contenu" class="textarea textarea-bordered" rows="4" required />
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Explication</span></label>
              <textarea v-model="form.explication" class="textarea textarea-bordered" rows="3" placeholder="Explication ou contexte culturel..." />
            </div>

            <div v-if="form.type_codimoi === 'citation'" class="form-control">
              <label class="label"><span class="label-text">Auteur originel</span></label>
              <input v-model="form.nom_auteur_originel" type="text" class="input input-bordered" placeholder="Nom de l'auteur de la citation">
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Territoire (UUID)</span></label>
                <input v-model="form.pays_id" type="text" class="input input-bordered" placeholder="ID du territoire d'origine">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Groupe ethnique</span></label>
                <input v-model="form.groupe_ethnique" type="text" class="input input-bordered" placeholder="Ex: Bamileke, Wolof, Zulu...">
              </div>
            </div>

            <div class="form-control">
              <label class="label"><span class="label-text">Couleur de fond</span></label>
              <div class="flex items-center gap-3">
                <input v-model="form.couleur_fond" type="color" class="w-12 h-10 rounded cursor-pointer border border-base-300">
                <input v-model="form.couleur_fond" type="text" class="input input-bordered input-sm w-32" placeholder="#FFFFFF">
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Image de couverture (URL)</span></label>
                <input v-model="form.image_couverture_url" type="text" class="input input-bordered" placeholder="https://...">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Image d'arriere-plan (URL)</span></label>
                <input v-model="form.image_arriere_plan_url" type="text" class="input input-bordered" placeholder="https://...">
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

      <!-- Onglet Tags -->
      <div v-if="ongletActif === 'tags'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="font-semibold mb-4">Tags associes</h3>
          <table class="table table-sm">
            <thead>
              <tr>
                <th>Tag</th>
                <th class="w-16">Actions</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="tag in postDetail.tags" :key="tag.tag_id">
                <td><span class="badge badge-sm badge-outline">{{ tag.tag_nom }}</span></td>
                <td>
                  <button class="btn btn-ghost btn-xs text-error" @click="supprimerTagHandler(tag.tag_id)">
                    <font-awesome-icon icon="trash" />
                  </button>
                </td>
              </tr>
              <tr v-if="!postDetail.tags.length">
                <td colspan="2" class="text-center text-base-content/50 py-4">Aucun tag associe</td>
              </tr>
            </tbody>
          </table>
          <div class="flex gap-2 mt-4">
            <input v-model="nouveauTagId" type="text" class="input input-bordered input-sm flex-1" placeholder="ID du tag a ajouter">
            <button class="btn btn-primary btn-sm" :disabled="!nouveauTagId" @click="ajouterTagHandler">
              <font-awesome-icon icon="plus" class="mr-1" /> Ajouter
            </button>
          </div>
        </div>
      </div>

      <!-- Onglet Commentaires -->
      <div v-if="ongletActif === 'commentaires'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="font-semibold mb-4">Commentaires (moderation)</h3>

          <div v-if="!commentaires.length" class="text-center text-base-content/50 py-8">
            Aucun commentaire pour ce codi-moi
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
                <p class="text-4xl font-bold text-success">{{ postDetail.nombre_likes }}</p>
                <p class="text-sm text-base-content/60">Likes</p>
              </div>
            </div>
            <div class="card bg-error/10 border border-error/20">
              <div class="card-body items-center text-center">
                <font-awesome-icon icon="thumbs-down" class="text-4xl text-error mb-2" />
                <p class="text-4xl font-bold text-error">{{ postDetail.nombre_dislikes }}</p>
                <p class="text-sm text-base-content/60">Dislikes</p>
              </div>
            </div>
          </div>
          <div class="mt-6">
            <div class="flex justify-between text-sm mb-1">
              <span>Ratio positif</span>
              <span>{{ postDetail.nombre_likes + postDetail.nombre_dislikes > 0 ? Math.round((postDetail.nombre_likes / (postDetail.nombre_likes + postDetail.nombre_dislikes)) * 100) : 0 }}%</span>
            </div>
            <progress
              class="progress progress-success w-full"
              :value="postDetail.nombre_likes"
              :max="postDetail.nombre_likes + postDetail.nombre_dislikes || 1"
            />
          </div>
        </div>
      </div>
    </template>
  </div>
</template>
