<script setup lang="ts">
definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { creer, loading, error } = useAdminUtilisateurs()
const router = useRouter()

const form = reactive({
  nom: '',
  prenom: '',
  email: '',
  mot_de_passe: '',
  telephone: '',
  genre: 'non_precise',
  role_id: '',
})

const erreurLocale = ref<string | null>(null)

const soumettre = async () => {
  erreurLocale.value = null

  if (!form.nom.trim() || !form.prenom.trim()) {
    erreurLocale.value = 'Nom et prenom requis'
    return
  }
  if (!form.email.trim()) {
    erreurLocale.value = 'Email requis'
    return
  }
  if (form.mot_de_passe.length < 8) {
    erreurLocale.value = 'Le mot de passe doit contenir au moins 8 caracteres'
    return
  }

  try {
    const body: any = {
      nom: form.nom.trim(),
      prenom: form.prenom.trim(),
      email: form.email.trim(),
      mot_de_passe: form.mot_de_passe,
    }
    if (form.telephone.trim()) body.telephone = form.telephone.trim()
    if (form.genre !== 'non_precise') body.genre = form.genre
    if (form.role_id) body.role_id = form.role_id

    await creer(body)
    router.push('/admin/utilisateurs')
  }
  catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la creation'
  }
}
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouvel utilisateur" sous-titre="Creer un compte utilisateur">
      <template #actions>
        <NuxtLink to="/admin/utilisateurs" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" />
          Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <div v-if="erreurLocale || error" class="alert alert-error mb-4">
          <font-awesome-icon icon="circle-exclamation" />
          <span>{{ erreurLocale || error }}</span>
        </div>

        <form @submit.prevent="soumettre" class="space-y-4">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Prenom *</span></label>
              <input v-model="form.prenom" type="text" class="input input-bordered" placeholder="Prenom" required>
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Nom *</span></label>
              <input v-model="form.nom" type="text" class="input input-bordered" placeholder="Nom" required>
            </div>
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Email *</span></label>
            <input v-model="form.email" type="email" class="input input-bordered" placeholder="email@exemple.com" required>
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Mot de passe *</span></label>
            <input v-model="form.mot_de_passe" type="password" class="input input-bordered" placeholder="Minimum 8 caracteres" required>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Telephone</span></label>
              <input v-model="form.telephone" type="tel" class="input input-bordered" placeholder="+243...">
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

          <div class="form-control">
            <label class="label"><span class="label-text">Role initial (optionnel)</span></label>
            <select v-model="form.role_id" class="select select-bordered">
              <option value="">Aucun role specifique</option>
            </select>
            <label class="label">
              <span class="label-text-alt text-base-content/50">Le role 'utilisateur' est attribue par defaut</span>
            </label>
          </div>

          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/utilisateurs" class="btn btn-ghost">Annuler</NuxtLink>
            <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" />
              Creer l'utilisateur
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
