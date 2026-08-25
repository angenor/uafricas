<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const router = useRouter()
const id = route.params.id as string

const {
  utilisateurDetail, loading, error,
  chargerDetail, modifier, changerEtat, validerCompteAdmin,
  assignerRole, retirerRole,
  assignerSpecialite, retirerSpecialite,
  ajouterPermission, retirerPermission,
} = useAdminUtilisateurs()

const ongletActif = ref('profil')
const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

// Formulaire profil
const form = reactive({
  nom: '',
  prenom: '',
  email: '',
  telephone: '',
  genre: '',
  date_naissance: '',
  fonction: '',
  localite: '',
  ville: '',
  biographie: '',
  langue_preferee: 'fr',
  bibliotheque_humain: false,
})

// Modal etat
const showEtatModal = ref(false)
const nouvelEtat = ref('')

// Role select
const nouveauRoleId = ref('')

// Specialite select
const nouvelleSpecialiteId = ref('')

const charger = async () => {
  await chargerDetail(id)
  if (utilisateurDetail.value) {
    const u = utilisateurDetail.value
    form.nom = u.nom
    form.prenom = u.prenom
    form.email = u.email
    form.telephone = u.telephone || ''
    form.genre = u.genre
    form.date_naissance = u.date_naissance || ''
    form.fonction = u.fonction || ''
    form.localite = u.localite || ''
    form.ville = u.ville || ''
    form.biographie = u.biographie || ''
    form.langue_preferee = u.langue_preferee
    form.bibliotheque_humain = u.bibliotheque_humain
  }
}

const sauvegarderProfil = async () => {
  saving.value = true
  erreurLocale.value = null
  successMsg.value = null
  try {
    await modifier(id, { ...form })
    successMsg.value = 'Profil mis a jour avec succes'
    setTimeout(() => { successMsg.value = null }, 3000)
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la sauvegarde'
  }
  finally {
    saving.value = false
  }
}

const executerChangementEtat = async () => {
  if (!nouvelEtat.value) return
  try {
    await changerEtat(id, nouvelEtat.value)
    showEtatModal.value = false
    await charger()
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
}

const basculerValidationCompte = async () => {
  if (!utilisateurDetail.value) return
  erreurLocale.value = null
  try {
    await validerCompteAdmin(id, !utilisateurDetail.value.compte_verifie_admin)
    await charger()
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
}

const ajouterRole = async () => {
  if (!nouveauRoleId.value) return
  try {
    await assignerRole(id, nouveauRoleId.value)
    nouveauRoleId.value = ''
    await charger()
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
}

const supprimerRole = async (roleId: string) => {
  try {
    await retirerRole(id, roleId)
    await charger()
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
}

const ajouterSpec = async () => {
  if (!nouvelleSpecialiteId.value) return
  try {
    await assignerSpecialite(id, nouvelleSpecialiteId.value)
    nouvelleSpecialiteId.value = ''
    await charger()
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
}

const supprimerSpec = async (specId: string) => {
  try {
    await retirerSpecialite(id, specId)
    await charger()
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur'
  }
}

onMounted(() => charger())
</script>

<template>
  <div>
    <AdminPageHeader
      :titre="utilisateurDetail ? `${utilisateurDetail.prenom} ${utilisateurDetail.nom}` : 'Chargement...'"
      sous-titre="Edition du profil utilisateur"
    >
      <template #actions>
        <NuxtLink to="/admin/utilisateurs" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" />
          Retour
        </NuxtLink>
        <template v-if="utilisateurDetail">
          <button
            v-if="utilisateurDetail.etat === 'actif'"
            class="btn btn-warning btn-sm"
            @click="nouvelEtat = 'suspendu'; showEtatModal = true"
          >
            <font-awesome-icon icon="pause" class="mr-1" />
            Suspendre
          </button>
          <button
            v-if="utilisateurDetail.etat === 'actif'"
            class="btn btn-error btn-sm"
            @click="nouvelEtat = 'bloque'; showEtatModal = true"
          >
            <font-awesome-icon icon="ban" class="mr-1" />
            Bloquer
          </button>
          <button
            v-if="utilisateurDetail.etat !== 'actif'"
            class="btn btn-success btn-sm"
            @click="nouvelEtat = 'actif'; showEtatModal = true"
          >
            <font-awesome-icon icon="check" class="mr-1" />
            Activer
          </button>
          <button
            class="btn btn-sm"
            :class="utilisateurDetail.compte_verifie_admin ? 'btn-outline btn-error' : 'btn-success'"
            @click="basculerValidationCompte"
          >
            <font-awesome-icon :icon="utilisateurDetail.compte_verifie_admin ? 'xmark' : 'user-shield'" class="mr-1" />
            {{ utilisateurDetail.compte_verifie_admin ? 'Retirer la validation' : 'Valider le compte' }}
          </button>
        </template>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !utilisateurDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="utilisateurDetail">
      <!-- Infos rapides -->
      <div class="flex items-center gap-4 mb-6">
        <div class="avatar placeholder">
          <div class="bg-neutral text-neutral-content rounded-full w-16 h-16">
            <span class="text-xl">{{ (utilisateurDetail.prenom?.[0] || '') + (utilisateurDetail.nom?.[0] || '') }}</span>
          </div>
        </div>
        <div>
          <h2 class="text-lg font-bold">{{ utilisateurDetail.prenom }} {{ utilisateurDetail.nom }}</h2>
          <p class="text-sm text-base-content/60">{{ utilisateurDetail.email }}</p>
          <div class="flex gap-2 mt-1">
            <AdminStatusBadge :statut="utilisateurDetail.etat" />
            <span v-if="utilisateurDetail.compte_verifie_admin" class="badge badge-xs badge-success gap-1">
              <font-awesome-icon icon="user-shield" />
              Compte validé
            </span>
            <span v-for="r in utilisateurDetail.roles" :key="r.id" class="badge badge-xs badge-outline">{{ r.slug }}</span>
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
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'profil' }" @click="ongletActif = 'profil'">
          <font-awesome-icon icon="user" class="mr-1" /> Profil
        </button>
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'roles' }" @click="ongletActif = 'roles'">
          <font-awesome-icon icon="shield-halved" class="mr-1" /> Roles
        </button>
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'specialites' }" @click="ongletActif = 'specialites'">
          <font-awesome-icon icon="book" class="mr-1" /> Specialites
        </button>
        <button role="tab" class="tab" :class="{ 'tab-active': ongletActif === 'permissions' }" @click="ongletActif = 'permissions'">
          <font-awesome-icon icon="key" class="mr-1" /> Permissions
        </button>
      </div>

      <!-- Onglet Profil -->
      <div v-if="ongletActif === 'profil'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <form @submit.prevent="sauvegarderProfil" class="space-y-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Prenom</span></label>
                <input v-model="form.prenom" type="text" class="input input-bordered">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Nom</span></label>
                <input v-model="form.nom" type="text" class="input input-bordered">
              </div>
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Email</span></label>
              <input v-model="form.email" type="email" class="input input-bordered">
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Telephone</span></label>
                <input v-model="form.telephone" type="tel" class="input input-bordered">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Genre</span></label>
                <select v-model="form.genre" class="select select-bordered">
                  <option value="non_precise">Non precise</option>
                  <option value="homme">Homme</option>
                  <option value="femme">Femme</option>
                  <option value="autre">Autre</option>
                </select>
              </div>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Date de naissance</span></label>
                <input v-model="form.date_naissance" type="date" class="input input-bordered">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Fonction</span></label>
                <input v-model="form.fonction" type="text" class="input input-bordered">
              </div>
            </div>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="form-control">
                <label class="label"><span class="label-text">Localite</span></label>
                <input v-model="form.localite" type="text" class="input input-bordered">
              </div>
              <div class="form-control">
                <label class="label"><span class="label-text">Ville</span></label>
                <input v-model="form.ville" type="text" class="input input-bordered">
              </div>
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Biographie</span></label>
              <textarea v-model="form.biographie" class="textarea textarea-bordered" rows="3" />
            </div>
            <div class="form-control">
              <label class="label cursor-pointer justify-start gap-2">
                <input v-model="form.bibliotheque_humain" type="checkbox" class="checkbox checkbox-sm">
                <span class="label-text">Bibliotheque humaine</span>
              </label>
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

      <!-- Onglet Roles -->
      <div v-if="ongletActif === 'roles'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="font-semibold mb-4">Roles assignes</h3>
          <table class="table table-sm">
            <thead>
              <tr>
                <th>Role</th>
                <th>Attribue par</th>
                <th>Date</th>
                <th class="w-16">Actions</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="role in utilisateurDetail.roles" :key="role.id">
                <td><span class="badge badge-sm badge-outline">{{ role.slug }}</span> {{ role.nom }}</td>
                <td>{{ role.attribue_par_nom || '-' }}</td>
                <td>{{ new Date(role.created_at).toLocaleDateString('fr-FR') }}</td>
                <td>
                  <button class="btn btn-ghost btn-xs text-error" @click="supprimerRole(role.id)">
                    <font-awesome-icon icon="trash" />
                  </button>
                </td>
              </tr>
              <tr v-if="!utilisateurDetail.roles.length">
                <td colspan="4" class="text-center text-base-content/50 py-4">Aucun role assigne</td>
              </tr>
            </tbody>
          </table>
          <div class="flex gap-2 mt-4">
            <input v-model="nouveauRoleId" type="text" class="input input-bordered input-sm flex-1" placeholder="ID du role a assigner">
            <button class="btn btn-primary btn-sm" :disabled="!nouveauRoleId" @click="ajouterRole">
              <font-awesome-icon icon="plus" class="mr-1" /> Assigner
            </button>
          </div>
        </div>
      </div>

      <!-- Onglet Specialites -->
      <div v-if="ongletActif === 'specialites'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="font-semibold mb-4">Specialites bibliotheque humaine</h3>
          <table class="table table-sm">
            <thead>
              <tr>
                <th>Specialite</th>
                <th>Slug</th>
                <th class="w-16">Actions</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="spec in utilisateurDetail.specialites" :key="spec.id">
                <td>{{ spec.nom }}</td>
                <td><code class="text-xs">{{ spec.slug }}</code></td>
                <td>
                  <button class="btn btn-ghost btn-xs text-error" @click="supprimerSpec(spec.id)">
                    <font-awesome-icon icon="trash" />
                  </button>
                </td>
              </tr>
              <tr v-if="!utilisateurDetail.specialites.length">
                <td colspan="3" class="text-center text-base-content/50 py-4">Aucune specialite assignee</td>
              </tr>
            </tbody>
          </table>
          <div class="flex gap-2 mt-4">
            <input v-model="nouvelleSpecialiteId" type="text" class="input input-bordered input-sm flex-1" placeholder="ID de la specialite">
            <button class="btn btn-primary btn-sm" :disabled="!nouvelleSpecialiteId" @click="ajouterSpec">
              <font-awesome-icon icon="plus" class="mr-1" /> Assigner
            </button>
          </div>
        </div>
      </div>

      <!-- Onglet Permissions -->
      <div v-if="ongletActif === 'permissions'" class="card bg-base-100 shadow-sm">
        <div class="card-body">
          <h3 class="font-semibold mb-4">Permissions specifiques</h3>
          <table class="table table-sm">
            <thead>
              <tr>
                <th>Permission</th>
                <th>Ressource</th>
                <th>Expire</th>
                <th class="w-16">Actions</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="perm in utilisateurDetail.permissions_specifiques" :key="perm.id">
                <td><code class="text-xs">{{ perm.permission_slug }}</code></td>
                <td><code class="text-xs">{{ perm.ressource_type }}:{{ perm.ressource_id }}</code></td>
                <td>{{ perm.expire_at ? new Date(perm.expire_at).toLocaleDateString('fr-FR') : '-' }}</td>
                <td>
                  <button class="btn btn-ghost btn-xs text-error" @click="retirerPermission(id, perm.id)">
                    <font-awesome-icon icon="trash" />
                  </button>
                </td>
              </tr>
              <tr v-if="!utilisateurDetail.permissions_specifiques.length">
                <td colspan="4" class="text-center text-base-content/50 py-4">Aucune permission specifique</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </template>

    <!-- Modal changement etat -->
    <dialog class="modal" :class="{ 'modal-open': showEtatModal }">
      <div class="modal-box max-w-md">
        <h3 class="font-bold text-lg">Confirmer le changement d'etat</h3>
        <p class="py-4">
          Voulez-vous vraiment changer l'etat de cet utilisateur en
          <strong>{{ nouvelEtat }}</strong> ?
        </p>
        <div class="modal-action">
          <button class="btn btn-ghost" @click="showEtatModal = false">Annuler</button>
          <button
            class="btn"
            :class="{
              'btn-success': nouvelEtat === 'actif',
              'btn-warning': nouvelEtat === 'suspendu',
              'btn-error': nouvelEtat === 'bloque',
            }"
            @click="executerChangementEtat"
          >
            Confirmer
          </button>
        </div>
      </div>
      <form method="dialog" class="modal-backdrop">
        <button @click="showEtatModal = false">fermer</button>
      </form>
    </dialog>
  </div>
</template>
