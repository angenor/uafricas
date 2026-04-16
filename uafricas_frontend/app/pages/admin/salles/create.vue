<script setup lang="ts">
import type { GroupeEthniqueOption } from '~/composables/useAdminSalles'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { creer, chargerGroupesEthniques, loading, error } = useAdminSalles()
const router = useRouter()

const form = reactive({
  titre: '',
  description: '',
  groupe_ethnique_id: '',
  langue_cible: '',
  langue_code: '',
  alphabet: '',
  dictionnaire_url: '',
})

const groupesEthniques = ref<GroupeEthniqueOption[]>([])
const chargementGroupes = ref(false)
const erreurLocale = ref<string | null>(null)

const groupeSelectionne = computed(() =>
  groupesEthniques.value.find(g => g.id === form.groupe_ethnique_id),
)

const groupeDejaAssocie = computed(() =>
  groupeSelectionne.value?.salle_active === true,
)

const chargerGroupes = async () => {
  chargementGroupes.value = true
  try {
    groupesEthniques.value = await chargerGroupesEthniques()
  } catch {
    erreurLocale.value = 'Impossible de charger les groupes ethniques'
  } finally {
    chargementGroupes.value = false
  }
}

const soumettre = async () => {
  erreurLocale.value = null
  if (!form.titre.trim()) {
    erreurLocale.value = 'Le titre de la salle est requis'
    return
  }
  if (!form.groupe_ethnique_id) {
    erreurLocale.value = 'Le groupe ethnique est requis'
    return
  }
  if (groupeDejaAssocie.value) {
    erreurLocale.value = 'Ce groupe ethnique a déjà une salle publique active'
    return
  }
  try {
    const body: any = {
      titre: form.titre.trim(),
      groupe_ethnique_id: form.groupe_ethnique_id,
    }
    if (form.description.trim()) body.description = form.description.trim()
    if (form.langue_cible.trim()) body.langue_cible = form.langue_cible.trim()
    if (form.langue_code.trim()) body.langue_code = form.langue_code.trim()
    if (form.alphabet.trim()) body.alphabet = form.alphabet.trim()
    if (form.dictionnaire_url.trim()) body.dictionnaire_url = form.dictionnaire_url.trim()
    await creer(body)
    router.push('/admin/salles')
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la création'
  }
}

onMounted(() => chargerGroupes())
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouvelle salle" sous-titre="Créer une salle AfroLang">
      <template #actions>
        <NuxtLink to="/admin/salles" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
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
          <div class="form-control">
            <label class="label"><span class="label-text">Titre de la salle *</span></label>
            <input v-model="form.titre" type="text" class="input input-bordered" required>
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Groupe ethnique *</span></label>
            <select
              v-model="form.groupe_ethnique_id"
              class="select select-bordered"
              :disabled="chargementGroupes"
              required
            >
              <option value="" disabled>{{ chargementGroupes ? 'Chargement...' : 'Sélectionner un groupe ethnique' }}</option>
              <option
                v-for="g in groupesEthniques"
                :key="g.id"
                :value="g.id"
                :disabled="g.salle_active"
              >
                {{ g.nom }}{{ g.pays_nom ? ` — ${g.pays_nom}` : '' }}{{ g.salle_active ? ' (déjà une salle active)' : '' }}
              </option>
            </select>
            <label v-if="groupeDejaAssocie" class="label">
              <span class="label-text-alt text-error">
                <font-awesome-icon icon="circle-exclamation" class="mr-1" />
                Ce groupe ethnique a déjà une salle publique active
              </span>
            </label>
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Description</span></label>
            <textarea v-model="form.description" class="textarea textarea-bordered" rows="3" />
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="form-control">
              <label class="label"><span class="label-text">Langue cible</span></label>
              <input v-model="form.langue_cible" type="text" class="input input-bordered" placeholder="Ex: Swahili, Wolof, Yoruba...">
            </div>
            <div class="form-control">
              <label class="label"><span class="label-text">Code langue</span></label>
              <input v-model="form.langue_code" type="text" class="input input-bordered" placeholder="Ex: sw, wo, yo...">
            </div>
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">Alphabet</span></label>
            <textarea
              v-model="form.alphabet"
              class="textarea textarea-bordered"
              rows="2"
              placeholder="Caractères de l'alphabet (affichés dans l'onglet Ressources)"
            />
          </div>

          <div class="form-control">
            <label class="label"><span class="label-text">URL dictionnaire</span></label>
            <input
              v-model="form.dictionnaire_url"
              type="url"
              class="input input-bordered"
              placeholder="https://..."
            >
          </div>

          <div class="flex justify-end gap-2 pt-4">
            <NuxtLink to="/admin/salles" class="btn btn-ghost">Annuler</NuxtLink>
            <button
              type="submit"
              class="btn btn-primary"
              :class="{ loading }"
              :disabled="loading || groupeDejaAssocie"
            >
              <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Créer
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>
